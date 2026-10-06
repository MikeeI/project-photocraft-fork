//! Autosave and crash recovery (architecture §9): the same incremental writer
//! saves a document *snapshot* into a recovery directory on a background
//! thread, so the UI never blocks.
//!
//! Layout: `<recovery_dir>/<key>.pcraft/` (directory bundles, so repeated
//! autosaves write only changed tiles) plus `<key>.json` with
//! [`RecoveryInfo`]. Native targets only (threads and a filesystem).

use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{SystemTime, UNIX_EPOCH};

use photocraft_doc::Document;
use serde::{Deserialize, Serialize};

use crate::store::write_atomic;
use crate::{PcraftWriter, Result, SaveOptions, SaveStats};

/// Sidecar describing an autosave.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoveryInfo {
    pub key: String,
    pub document_name: String,
    /// Where the user last saved the document, if anywhere.
    pub original_path: Option<String>,
    /// Seconds since the Unix epoch.
    pub saved_at: u64,
    pub revision: u64,
}

/// One recoverable document found by [`list_recovery`].
#[derive(Debug, Clone, PartialEq)]
pub struct RecoveryEntry {
    pub info: RecoveryInfo,
    pub bundle: PathBuf,
}

struct Job {
    snapshot: Arc<Document>,
    info: RecoveryInfo,
    opts: SaveOptions,
}

#[derive(Debug, Clone)]
struct Completion {
    revision: u64,
    result: std::result::Result<SaveStats, String>,
}

const AUTOSAVE_WORKER_UNAVAILABLE: &str = "autosave worker is unavailable";
const AUTOSAVE_WORKER_PANICKED: &str = "autosave worker panicked";

/// Background autosaver for one document. Requests are coalesced: if saves
/// arrive faster than they complete, only the newest snapshot is written.
pub struct Autosaver {
    dir: PathBuf,
    key: String,
    tx: Option<Sender<Job>>,
    completions: Receiver<Completion>,
    handle: Option<JoinHandle<()>>,
    pending_revision: Cell<Option<u64>>,
    last: Arc<Mutex<Option<Completion>>>,
}

fn sanitize(key: &str) -> String {
    let s: String = key.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '_' }).collect();
    if s.is_empty() { "untitled".into() } else { s }
}

impl Autosaver {
    /// `key` identifies the document across autosaves (e.g. its DocId).
    pub fn new(recovery_dir: impl Into<PathBuf>, key: &str) -> Self {
        let dir = recovery_dir.into();
        let key = sanitize(key);
        let (tx, rx) = mpsc::channel::<Job>();
        let (completion_tx, completions) = mpsc::channel();
        let bundle = dir.join(format!("{key}.pcraft"));
        let sidecar = dir.join(format!("{key}.json"));
        let last = Arc::new(Mutex::new(None));
        let last2 = last.clone();
        let handle = std::thread::Builder::new().name(format!("autosave-{key}")).spawn(move || worker(rx, bundle, sidecar, completion_tx, last2)).ok();
        Autosaver { dir, key, tx: Some(tx), completions, handle, pending_revision: Cell::new(None), last }
    }

    pub fn bundle_path(&self) -> PathBuf {
        self.dir.join(format!("{}.pcraft", self.key))
    }

    /// Queue a snapshot for saving; success means queued, not yet persisted.
    /// `revision` increases for this key, so a completed newer snapshot covers older queued requests.
    ///
    /// # Errors
    ///
    /// Returns an error if the background worker cannot accept the snapshot.
    pub fn request(&self, snapshot: Arc<Document>, revision: u64, original_path: Option<String>, opts: SaveOptions) -> Result<()> {
        let info = RecoveryInfo {
            key: self.key.clone(),
            document_name: snapshot.name.clone(),
            original_path,
            saved_at: SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0),
            revision,
        };
        let job = Job { snapshot, info, opts };
        let tx = self.tx.as_ref().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::BrokenPipe, AUTOSAVE_WORKER_UNAVAILABLE))?;
        tx.send(job).map_err(|_| std::io::Error::new(std::io::ErrorKind::BrokenPipe, AUTOSAVE_WORKER_UNAVAILABLE))?;
        self.pending_revision.set(Some(revision));
        Ok(())
    }

    /// Result of the most recent completed save.
    pub fn last_result(&self) -> Option<std::result::Result<SaveStats, String>> {
        self.last.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map(|completion| completion.result.clone())
    }

    /// Take newly completed saves; a disconnected worker fails any unreported request.
    pub fn take_results(&mut self) -> Vec<(u64, std::result::Result<SaveStats, String>)> {
        let mut out = Vec::new();
        loop {
            match self.completions.try_recv() {
                Ok(completion) => {
                    // Coalesced revisions are covered by the newest completed snapshot.
                    if self.pending_revision.get().is_some_and(|pending| completion.revision >= pending) {
                        self.pending_revision.set(None);
                    }
                    out.push((completion.revision, completion.result));
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    if let Some(revision) = self.pending_revision.take() {
                        let completion = Completion { revision, result: Err(AUTOSAVE_WORKER_UNAVAILABLE.to_string()) };
                        store_completion(&self.last, completion.clone());
                        out.push((revision, completion.result));
                    }
                    break;
                }
            }
        }
        out
    }

    /// Finish pending saves and stop the thread.
    pub fn flush(mut self) -> Option<std::result::Result<SaveStats, String>> {
        let shutdown = self.shutdown();
        let _ = self.take_results();
        if let Err(error) = shutdown {
            return Some(Err(error));
        }
        self.last_result()
    }

    /// The document was saved normally or closed: drop its recovery data.
    pub fn discard(mut self) -> Result<()> {
        self.shutdown().map_err(std::io::Error::other)?;
        remove_entry(&self.dir, &self.key)
    }

    fn shutdown(&mut self) -> std::result::Result<(), String> {
        self.tx.take();
        if let Some(handle) = self.handle.take() {
            handle.join().map_err(|_| AUTOSAVE_WORKER_PANICKED.to_string())?;
        }
        Ok(())
    }
}

impl Drop for Autosaver {
    fn drop(&mut self) {
        // Drop cannot report worker failure; callers needing that result use `flush` or `discard`.
        let _ = self.shutdown();
    }
}

fn store_completion(last: &Mutex<Option<Completion>>, completion: Completion) {
    let mut latest = last.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    *latest = Some(completion);
}

fn worker(rx: Receiver<Job>, bundle: PathBuf, sidecar: PathBuf, completion_tx: Sender<Completion>, last: Arc<Mutex<Option<Completion>>>) {
    let mut writer = PcraftWriter::new();
    while let Ok(mut job) = rx.recv() {
        // Coalesce: skip to the newest queued snapshot.
        while let Ok(newer) = rx.try_recv() {
            job = newer;
        }
        let revision = job.info.revision;
        let result = (|| {
            std::fs::create_dir_all(&bundle)?;
            let stats = writer.save_dir(&job.snapshot, &bundle, &job.opts)?;
            write_atomic(&sidecar, &serde_json::to_vec_pretty(&job.info)?)?;
            Ok(stats)
        })()
        .map_err(|error: crate::FormatError| error.to_string());
        let completion = Completion { revision, result };
        store_completion(&last, completion.clone());
        if completion_tx.send(completion).is_err() {
            break;
        }
    }
}

/// Recoverable documents in `recovery_dir`, newest first.
pub fn list_recovery(recovery_dir: &Path) -> Vec<RecoveryEntry> {
    let mut out = Vec::new();
    let Ok(rd) = std::fs::read_dir(recovery_dir) else {
        return out;
    };
    for e in rd.flatten() {
        let p = e.path();
        if p.extension().is_some_and(|x| x == "json")
            && let Ok(bytes) = std::fs::read(&p)
            && let Ok(info) = serde_json::from_slice::<RecoveryInfo>(&bytes)
        {
            let bundle = recovery_dir.join(format!("{}.pcraft", info.key));
            if bundle.join(crate::store::MANIFEST).is_file() {
                out.push(RecoveryEntry { info, bundle });
            }
        }
    }
    out.sort_by(|a, b| b.info.saved_at.cmp(&a.info.saved_at).then(a.info.key.cmp(&b.info.key)));
    out
}

/// Load a recovered document.
pub fn recover(entry: &RecoveryEntry) -> Result<Document> {
    crate::load_path(&entry.bundle)
}

/// Delete a recovery entry.
pub fn discard_recovery(recovery_dir: &Path, entry: &RecoveryEntry) -> Result<()> {
    remove_entry(recovery_dir, &entry.info.key)
}

fn remove_entry(dir: &Path, key: &str) -> Result<()> {
    let bundle = dir.join(format!("{key}.pcraft"));
    if bundle.exists() {
        std::fs::remove_dir_all(bundle)?;
    }
    let sidecar = dir.join(format!("{key}.json"));
    if sidecar.exists() {
        std::fs::remove_file(sidecar)?;
    }
    Ok(())
}
