//! # utqc-agent
//!
//! Agent governance hooks for proof release.
//! Every circuit release requires agent approval.

use utqc_worm::WormSeal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Agent governance error.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum AgentError {
    #[error("insufficient permissions: requires {0}")]
    InsufficientPermissions(String),

    #[error("governance vote failed")]
    VoteFailed,

    #[error("agent not found: {0}")]
    AgentNotFound(String),
}

/// Agent identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIdentity {
    /// Agent name.
    pub name: String,
    /// Agent role.
    pub role: AgentRole,
    /// Permissions.
    pub permissions: Vec<String>,
}

/// Agent roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentRole {
    /// Can compile circuits.
    Compiler,
    /// Can verify circuits.
    Verifier,
    /// Can release artifacts.
    Releaser,
    /// Can govern.
    Governor,
}

/// Governance vote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceVote {
    /// Agent that voted.
    pub agent: String,
    /// Approval.
    pub approved: bool,
    /// Reason.
    pub reason: String,
}

/// Governance record for an artifact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceRecord {
    /// Artifact identifier.
    pub artifact: String,
    /// Votes collected.
    pub votes: Vec<GovernanceVote>,
    /// WORM seal for this record.
    pub seal: WormSeal,
}

impl GovernanceRecord {
    /// Create a new governance record.
    pub fn new(artifact: &str) -> Self {
        let seal = WormSeal::seal("GOVERNANCE", artifact, 0);
        Self { artifact: artifact.to_string(), votes: Vec::new(), seal }
    }

    /// Add a vote.
    pub fn add_vote(&mut self, vote: GovernanceVote) {
        self.votes.push(vote);
    }

    /// Check if approved (majority must approve).
    pub fn is_approved(&self) -> bool {
        let total = self.votes.len();
        if total == 0 {
            return false;
        }
        let approved = self.votes.iter().filter(|v| v.approved).count();
        approved > total / 2
    }
}

/// Agent governance manager.
pub struct AgentGovernance {
    /// Registered agents.
    agents: Vec<AgentIdentity>,
}

impl AgentGovernance {
    /// Create a new governance manager.
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    /// Register an agent.
    pub fn register(&mut self, agent: AgentIdentity) {
        self.agents.push(agent);
    }

    /// Find an agent by name.
    pub fn find(&self, name: &str) -> Option<&AgentIdentity> {
        self.agents.iter().find(|a| a.name == name)
    }

    /// Check if agent has permission.
    pub fn check_permission(&self, agent_name: &str, permission: &str) -> Result<(), AgentError> {
        let agent = self.find(agent_name)
            .ok_or_else(|| AgentError::AgentNotFound(agent_name.to_string()))?;
        if agent.permissions.contains(&permission.to_string()) {
            Ok(())
        } else {
            Err(AgentError::InsufficientPermissions(permission.to_string()))
        }
    }

    /// Collect votes for an artifact release.
    pub fn collect_votes(&self, artifact: &str) -> GovernanceRecord {
        let mut record = GovernanceRecord::new(artifact);
        for agent in &self.agents {
            let approved = agent.role == AgentRole::Governor
                || agent.role == AgentRole::Releaser
                || agent.permissions.contains(&"release".to_string());
            record.add_vote(GovernanceVote {
                agent: agent.name.clone(),
                approved,
                reason: if approved { "Approved".to_string() } else { "Insufficient role".to_string() },
            });
        }
        record
    }
}

impl Default for AgentGovernance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_governance() {
        let mut gov = AgentGovernance::new();
        gov.register(AgentIdentity {
            name: "compiler".to_string(),
            role: AgentRole::Releaser,
            permissions: vec!["compile".to_string(), "release".to_string()],
        });
        gov.register(AgentIdentity {
            name: "governor".to_string(),
            role: AgentRole::Governor,
            permissions: vec!["release".to_string(), "govern".to_string()],
        });

        let record = gov.collect_votes("test-artifact");
        assert!(record.is_approved());
    }

    #[test]
    fn test_permission_check() {
        let mut gov = AgentGovernance::new();
        gov.register(AgentIdentity {
            name: "alice".to_string(),
            role: AgentRole::Compiler,
            permissions: vec!["compile".to_string()],
        });

        assert!(gov.check_permission("alice", "compile").is_ok());
        assert!(gov.check_permission("alice", "release").is_err());
    }
}
