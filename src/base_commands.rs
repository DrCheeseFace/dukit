use crate::{get_editor, DisplayableCliCommand, TEMP_FILE_PATH};
use std::{
    fs::{self, File},
    io::Write,
    process::Command,
};

pub enum BaseCliCommands {
    Status,
    CurrentBranch,
    RemoteBranch,
    OpenEditor,
    Todo,
}

impl BaseCliCommands {
    fn get_cli_command(&self) -> String {
        match self {
            BaseCliCommands::Status => "git status -s".to_string(),
            BaseCliCommands::CurrentBranch => "git branch --show-current".to_string(),
            BaseCliCommands::RemoteBranch => {
                "git status -uno | grep -E 'Your branch is (ahead|behind|up to date)'".to_string()
            }
            BaseCliCommands::OpenEditor => get_editor(),
            BaseCliCommands::Todo => "echo 'hi :3'".to_string(),
        }
    }

    pub fn run(self, stdin: Option<String>) -> String {
        match self {
            BaseCliCommands::OpenEditor => self.open_editor(stdin.unwrap_or_default()),
            _ => self.run_generic(),
        }
    }

    /// opens editor with given stdin to display to user
    /// returns a string of the saved file on exit
    fn open_editor(self, stdin: String) -> String {
        let mut file = File::create(TEMP_FILE_PATH).unwrap();
        file.write_all(stdin.as_bytes()).unwrap();
        drop(file);

        Command::new(get_editor())
            .arg(TEMP_FILE_PATH)
            .spawn()
            .expect("couldnt open temp file with editor")
            .wait()
            .expect("bad exit code from closing editor");

        fs::read_to_string(TEMP_FILE_PATH).unwrap()
    }

    /// runs a generic cli command
    /// returns a string of the output
    fn run_generic(self) -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg(self.get_cli_command())
            .output()
            .expect("failed to execute process");
        let displayable_output = DisplayableCliCommand(output);
        displayable_output.to_string()
    }
}
