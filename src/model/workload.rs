use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Manifest as read from a `mini apply` YAML file (see project brief format).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadManifest {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub kind: String,
    pub metadata: WorkloadMetadata,
    pub spec: WorkloadSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadMetadata {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadSpec {
    pub command: Vec<String>,
    #[serde(default)]
    pub rootfs: Option<String>,
    #[serde(default = "default_replicas")]
    pub replicas: u32,
    #[serde(default)]
    pub resources: ResourceRequirements,
    #[serde(rename = "restartPolicy", default)]
    pub restart_policy: RestartPolicy,
    #[serde(default)]
    pub network: Option<NetworkSpec>,
}

fn default_replicas() -> u32 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceRequirements {
    #[serde(default)]
    pub cpu: Option<String>,
    #[serde(default)]
    pub memory: Option<String>,
    #[serde(default)]
    pub pids: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum RestartPolicy {
    Never,
    OnFailure,
    #[default]
    Always,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSpec {
    #[serde(rename = "exposePort", default)]
    pub expose_port: Option<u16>,
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("kind must be 'Workload', got '{0}'")]
    InvalidKind(String),
    #[error("workload name must not be empty")]
    EmptyName,
    #[error("workload name '{0}' is invalid: use lowercase alphanumeric characters and '-' only, no leading/trailing '-'")]
    InvalidName(String),
    #[error("workload command must not be empty")]
    EmptyCommand,
    #[error("replicas must be at least 1")]
    ZeroReplicas,
}

impl WorkloadManifest {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.kind != "Workload" {
            return Err(ValidationError::InvalidKind(self.kind.clone()));
        }
        let name = self.metadata.name.trim();
        if name.is_empty() {
            return Err(ValidationError::EmptyName);
        }
        let valid_name = name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            && !name.starts_with('-')
            && !name.ends_with('-');
        if !valid_name {
            return Err(ValidationError::InvalidName(name.to_string()));
        }
        if self.spec.command.is_empty() {
            return Err(ValidationError::EmptyCommand);
        }
        if self.spec.replicas == 0 {
            return Err(ValidationError::ZeroReplicas);
        }
        Ok(())
    }
}

/// A persisted workload record: validated spec plus identity/versioning metadata.
/// The id is immutable for the workload's lifetime; `version` increments on every `apply`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workload {
    pub id: uuid::Uuid,
    pub name: String,
    pub spec: WorkloadSpec,
    pub version: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Workload {
    pub fn new(manifest: WorkloadManifest) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            name: manifest.metadata.name,
            spec: manifest.spec,
            version: 1,
            created_at: now,
            updated_at: now,
        }
    }

    /// Applies a new manifest to this existing record: same id, bumped version.
    pub fn apply_update(&mut self, manifest: WorkloadManifest) {
        self.spec = manifest.spec;
        self.version += 1;
        self.updated_at = chrono::Utc::now();
    }
}
