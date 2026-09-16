use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Node health as observed via heartbeats (introduced in Stage 4/6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeHealth {
    Ready,
    NotReady,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeCapacity {
    pub cpu_millis: u64,
    pub memory_bytes: u64,
    pub pids: u32,
}

/// A machine or agent that can run instances. Unused until Stage 4 (node agent).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: uuid::Uuid,
    pub name: String,
    pub labels: HashMap<String, String>,
    pub capacity: NodeCapacity,
    pub health: NodeHealth,
    pub last_heartbeat: Option<chrono::DateTime<chrono::Utc>>,
}

impl Node {
    pub fn new(name: impl Into<String>, capacity: NodeCapacity) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: name.into(),
            labels: HashMap::new(),
            capacity,
            health: NodeHealth::Unknown,
            last_heartbeat: None,
        }
    }
}
