use crate::model::Workload;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("io error at {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to (de)serialize record at {path}: {source}")]
    Serde {
        path: PathBuf,
        source: serde_json::Error,
    },
    #[error("workload '{0}' not found")]
    NotFound(String),
}

/// Flat-file JSON store for workload records: one file per workload, keyed by id.
/// Deliberately simple for Stage 0-4; replaced by SQLite in Stage 5.
pub struct WorkloadStore {
    dir: PathBuf,
}

impl WorkloadStore {
    pub fn open(data_dir: &Path) -> Result<Self, StoreError> {
        let dir = data_dir.join("workloads");
        fs::create_dir_all(&dir).map_err(|e| StoreError::Io {
            path: dir.clone(),
            source: e,
        })?;
        Ok(Self { dir })
    }

    fn path_for(&self, id: Uuid) -> PathBuf {
        self.dir.join(format!("{id}.json"))
    }

    pub fn save(&self, workload: &Workload) -> Result<(), StoreError> {
        let path = self.path_for(workload.id);
        let json = serde_json::to_vec_pretty(workload).map_err(|e| StoreError::Serde {
            path: path.clone(),
            source: e,
        })?;
        fs::write(&path, json).map_err(|e| StoreError::Io { path, source: e })
    }

    pub fn list(&self) -> Result<Vec<Workload>, StoreError> {
        let mut out = Vec::new();
        for entry in fs::read_dir(&self.dir).map_err(|e| StoreError::Io {
            path: self.dir.clone(),
            source: e,
        })? {
            let entry = entry.map_err(|e| StoreError::Io {
                path: self.dir.clone(),
                source: e,
            })?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            out.push(Self::read(&path)?);
        }
        out.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(out)
    }

    fn read(path: &Path) -> Result<Workload, StoreError> {
        let bytes = fs::read(path).map_err(|e| StoreError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;
        serde_json::from_slice(&bytes).map_err(|e| StoreError::Serde {
            path: path.to_path_buf(),
            source: e,
        })
    }

    /// Finds a workload by id (as a UUID string) or exact name.
    pub fn find(&self, id_or_name: &str) -> Result<Workload, StoreError> {
        if let Ok(id) = Uuid::parse_str(id_or_name) {
            let path = self.path_for(id);
            if path.exists() {
                return Self::read(&path);
            }
        }
        self.list()?
            .into_iter()
            .find(|w| w.name == id_or_name)
            .ok_or_else(|| StoreError::NotFound(id_or_name.to_string()))
    }

    pub fn delete(&self, id_or_name: &str) -> Result<Workload, StoreError> {
        let workload = self.find(id_or_name)?;
        let path = self.path_for(workload.id);
        fs::remove_file(&path).map_err(|e| StoreError::Io { path, source: e })?;
        Ok(workload)
    }
}
