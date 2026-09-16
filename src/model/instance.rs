use serde::{Deserialize, Serialize};

/// Lifecycle state of a single instance:
///
/// ```text
/// Pending -> Assigned -> Starting -> Running -> Succeeded | Failed
///                                  \-> Stopping -> Stopped
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstanceState {
    Pending,
    Assigned,
    Starting,
    Running,
    Succeeded,
    Failed,
    Stopping,
    Stopped,
}

impl InstanceState {
    /// Whether transitioning from `self` to `next` is a legal edge in the state machine.
    pub fn can_transition_to(self, next: InstanceState) -> bool {
        use InstanceState::*;
        matches!(
            (self, next),
            (Pending, Assigned)
                | (Assigned, Starting)
                | (Starting, Running)
                | (Starting, Failed)
                | (Running, Succeeded)
                | (Running, Failed)
                | (Running, Stopping)
                | (Stopping, Stopped)
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub id: uuid::Uuid,
    pub workload_id: uuid::Uuid,
    pub node_id: Option<uuid::Uuid>,
    pub state: InstanceState,
    pub restart_count: u32,
    pub exit_code: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Instance {
    pub fn new(workload_id: uuid::Uuid) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            workload_id,
            node_id: None,
            state: InstanceState::Pending,
            restart_count: 0,
            exit_code: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn transition(&mut self, next: InstanceState) -> Result<(), String> {
        if !self.state.can_transition_to(next) {
            return Err(format!(
                "illegal transition {:?} -> {:?} for instance {}",
                self.state, next, self.id
            ));
        }
        self.state = next;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }
}
