use crate::SWITCHED_BRANCH;
use std::process::Output;

#[derive(Debug)]
pub struct DisplayableCliCommand(pub Output);

impl std::fmt::Display for DisplayableCliCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stdout = String::from_utf8_lossy(&self.0.stdout);
        let stderr = String::from_utf8_lossy(&self.0.stderr);

        if stderr.is_empty() && !stderr.contains(SWITCHED_BRANCH) {
            return writeln!(f, "{}", stdout);
        }
        writeln!(f, "{}", stderr)
    }
}
