pub mod base_commands;
pub mod duck_commands;
use std::{env::var, process::Output};

const TEMP_FILE_PATH: &str = "/tmp/duk.txt";

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


/// gets the default editor the system 
fn get_editor() -> String {
    var("EDITOR").unwrap_or("vi".to_string())
}


