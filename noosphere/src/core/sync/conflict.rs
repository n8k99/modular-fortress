//! Conflict resolution for sync operations
//!
//! Handles merge conflicts when syncing between databases.

use chrono::NaiveDateTime;

/// Strategy for resolving conflicts
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConflictStrategy {
    /// Most recent modification wins
    LastWriteWins,
    /// Remote (server) always wins
    RemoteWins,
    /// Local always wins
    LocalWins,
    /// Flag for manual resolution
    Manual,
}

impl Default for ConflictStrategy {
    fn default() -> Self {
        ConflictStrategy::LastWriteWins
    }
}

/// A detected conflict between records
#[derive(Debug, Clone)]
pub struct Conflict<T> {
    /// ID of the conflicting record
    pub id: i32,
    /// Local version
    pub local: T,
    /// Remote version
    pub remote: T,
    /// Local modification time
    pub local_modified: Option<NaiveDateTime>,
    /// Remote modification time
    pub remote_modified: Option<NaiveDateTime>,
}

/// Conflict resolver
pub struct ConflictResolver {
    strategy: ConflictStrategy,
    /// Conflicts flagged for manual resolution
    pub pending_conflicts: Vec<PendingConflict>,
}

/// A conflict pending manual resolution
#[derive(Debug, Clone)]
pub struct PendingConflict {
    pub table: String,
    pub record_id: i32,
    pub local_modified: Option<NaiveDateTime>,
    pub remote_modified: Option<NaiveDateTime>,
    pub description: String,
}

impl ConflictResolver {
    pub fn new(strategy: ConflictStrategy) -> Self {
        Self {
            strategy,
            pending_conflicts: Vec::new(),
        }
    }

    /// Resolve a conflict based on the configured strategy
    pub fn resolve<T: Clone>(
        &mut self,
        conflict: &Conflict<T>,
        table: &str,
    ) -> Resolution<T> {
        match self.strategy {
            ConflictStrategy::LastWriteWins => {
                match (conflict.local_modified, conflict.remote_modified) {
                    (Some(local), Some(remote)) => {
                        if local > remote {
                            Resolution::UseLocal(conflict.local.clone())
                        } else {
                            Resolution::UseRemote(conflict.remote.clone())
                        }
                    }
                    (Some(_), None) => Resolution::UseLocal(conflict.local.clone()),
                    (None, Some(_)) => Resolution::UseRemote(conflict.remote.clone()),
                    (None, None) => Resolution::UseRemote(conflict.remote.clone()),
                }
            }
            ConflictStrategy::RemoteWins => {
                Resolution::UseRemote(conflict.remote.clone())
            }
            ConflictStrategy::LocalWins => {
                Resolution::UseLocal(conflict.local.clone())
            }
            ConflictStrategy::Manual => {
                self.pending_conflicts.push(PendingConflict {
                    table: table.to_string(),
                    record_id: conflict.id,
                    local_modified: conflict.local_modified,
                    remote_modified: conflict.remote_modified,
                    description: format!("Conflict on {} record {}", table, conflict.id),
                });
                Resolution::Pending
            }
        }
    }

    /// Get pending conflicts that need manual resolution
    pub fn get_pending(&self) -> &[PendingConflict] {
        &self.pending_conflicts
    }

    /// Clear resolved conflicts
    pub fn clear_pending(&mut self) {
        self.pending_conflicts.clear();
    }

    /// Check if there are pending conflicts
    pub fn has_pending(&self) -> bool {
        !self.pending_conflicts.is_empty()
    }
}

/// Result of conflict resolution
#[derive(Debug, Clone)]
pub enum Resolution<T> {
    /// Use the local version
    UseLocal(T),
    /// Use the remote version
    UseRemote(T),
    /// Conflict is pending manual resolution
    Pending,
}

impl<T> Resolution<T> {
    pub fn is_pending(&self) -> bool {
        matches!(self, Resolution::Pending)
    }
}
