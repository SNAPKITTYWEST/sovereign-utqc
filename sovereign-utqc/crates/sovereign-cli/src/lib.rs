//! # sovereign-cli
//!
//! CLI entry points for the sovereign compute pipeline.

use thiserror::Error;

/// CLI error.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CliError {
    #[error("missing argument: {0}")]
    MissingArgument(String),

    #[error("invalid command: {0}")]
    InvalidCommand(String),
}

/// CLI commands.
#[derive(Debug, Clone)]
pub enum Command {
    /// Compile a circuit.
    Compile { input: String },
    /// Verify a circuit.
    Verify { input: String },
    /// Seal an artifact.
    Seal { label: String, payload: String },
    /// Show version.
    Version,
}

impl Command {
    /// Parse from args.
    pub fn from_args(args: &[String]) -> Result<Self, CliError> {
        match args.get(1).map(|s| s.as_str()) {
            Some("compile") => {
                let input = args.get(2).ok_or_else(|| CliError::MissingArgument("input".to_string()))?;
                Ok(Command::Compile { input: input.clone() })
            }
            Some("verify") => {
                let input = args.get(2).ok_or_else(|| CliError::MissingArgument("input".to_string()))?;
                Ok(Command::Verify { input: input.clone() })
            }
            Some("seal") => {
                let label = args.get(2).ok_or_else(|| CliError::MissingArgument("label".to_string()))?;
                let payload = args.get(3).ok_or_else(|| CliError::MissingArgument("payload".to_string()))?;
                Ok(Command::Seal { label: label.clone(), payload: payload.clone() })
            }
            Some("version") => Ok(Command::Version),
            Some(cmd) => Err(CliError::InvalidCommand(cmd.to_string())),
            None => Err(CliError::MissingArgument("command".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let args = vec!["sovereign".to_string(), "version".to_string()];
        assert!(matches!(Command::from_args(&args).unwrap(), Command::Version));
    }

    #[test]
    fn test_parse_compile() {
        let args = vec!["sovereign".to_string(), "compile".to_string(), "circuit.json".to_string()];
        let cmd = Command::from_args(&args).unwrap();
        match cmd {
            Command::Compile { input } => assert_eq!(input, "circuit.json"),
            _ => panic!("wrong command"),
        }
    }
}
