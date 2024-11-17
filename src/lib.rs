pub mod base_commands;
pub mod duck_commands;
pub mod errors;
use std::process::Output;

const TEMP_FILE_PATH: &str = "/tmp/duk.md";

const LINE_SEPERATOR: &str = "# -------------------------------------";
const INTERACTIVE_ADD_HELP: &str = "# Selected files to be staged like so below V\n# [x] file.txt\n# Lines begining with (#) will be ignored";

#[derive(Debug)]
pub struct DisplayableCliCommand(Output);

impl std::fmt::Display for DisplayableCliCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stdout = String::from_utf8_lossy(&self.0.stdout);
        let stderr = String::from_utf8_lossy(&self.0.stderr);

        if stderr.is_empty() {
            return writeln!(f, "{}", stdout);
        }
        return writeln!(f, "{}", stderr);
    }
}
