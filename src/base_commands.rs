use crate::errors::DuckErrors;
use crate::{DisplayableCliCommand, TEMP_FILE_PATH};
use std::{
    env::var,
    fs::{self, File},
    io::Write,
    process::Command,
};

pub enum BaseCliCommands {
    Status,
    CurrentBranch,
    RemoteBranch,
    OpenEditor,
    AddFile,
    Todo,
}

impl BaseCliCommands {
    /// runs command and returns output as a string
    pub fn run(self, stdin: Option<String>) -> Result<String, DuckErrors> {
        match self {
            BaseCliCommands::OpenEditor => self.open_editor(stdin.unwrap_or_default()),
            BaseCliCommands::AddFile => self.add_file(stdin.unwrap_or_default()),
            _ => self.run_generic_command(),
        }
    }

    /// gets the cli command as a string
    fn get_cli_command(&self) -> String {
        match self {
            BaseCliCommands::Status => "git status -s".to_string(),
            BaseCliCommands::CurrentBranch => "git branch --show-current".to_string(),
            BaseCliCommands::RemoteBranch => {
                "git status -uno | grep -E 'Your branch is (ahead|behind|up to date)'".to_string()
            }
            BaseCliCommands::OpenEditor => self.get_editor(),
            BaseCliCommands::AddFile => "git add".to_string(),
            BaseCliCommands::Todo => "echo 'hi :3'".to_string(),
        }
    }

    /// opens editor with given stdin to display to user
    /// returns a string of the saved file on exit
    fn open_editor(self, stdin: String) -> Result<String, DuckErrors> {
        let mut file = File::create(TEMP_FILE_PATH).unwrap();
        file.write_all(stdin.as_bytes()).unwrap();
        drop(file);

        Command::new(self.get_editor())
            .arg(TEMP_FILE_PATH)
            .spawn()
            .map_err(|_| DuckErrors::TODO)?
            .wait()
            .map_err(|_| DuckErrors::TODO)?;

        if let Ok(out) = fs::read_to_string(TEMP_FILE_PATH) {
            return Ok(out);
        }
        Err(DuckErrors::TODO)
    }

    /// runs git add {stdin}
    fn add_file(self, file_name: String) -> Result<String, DuckErrors> {
        let cmd_to_add_file = self.get_cli_command() + " " + &file_name;
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd_to_add_file)
            .output()
            .map_err(|_| DuckErrors::TODO)?;

        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() {
            return Err(DuckErrors::TODO);
        }

        let displayable_output = DisplayableCliCommand(output);
        Ok(displayable_output.to_string())
    }

    /// runs a generic cli command
    /// returns a string of the output
    fn run_generic_command(self) -> Result<String, DuckErrors> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(self.get_cli_command())
            .output()
            .map_err(|_| DuckErrors::TODO)?;

        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stderr.is_empty() {
            return Err(DuckErrors::TODO);
        }

        let displayable_output = DisplayableCliCommand(output);
        Ok(displayable_output.to_string())
    }

    /// gets the default editor the system
    fn get_editor(&self) -> String {
        var("EDITOR").unwrap_or("vi".to_string())
    }
}
