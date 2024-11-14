use std::{
    fs::{self, File},
    io::Write,
    process::Command,
};

use crate::DisplayableCliCommand;

pub enum BaseCliCommands {
    Status,
    CurrentBranch,
    RemoteBranch,
    OpenEditor,
    Todo,
}

impl BaseCliCommands {
    pub fn get_cli_command(&self) -> &str {
        match self {
            BaseCliCommands::Status => "git status -s",
            BaseCliCommands::CurrentBranch => "git branch --show-current",
            BaseCliCommands::RemoteBranch => {
                "git status -uno | grep -E 'Your branch is (ahead|behind|up to date)'"
            }
            BaseCliCommands::OpenEditor => "\"${EDITOR:-vi}\"",
            BaseCliCommands::Todo => "echo 'hi :3'",
        }
    }

    pub fn run(self, stdin: Option<String>) -> String {
        match self {
            BaseCliCommands::OpenEditor => self.open_editor(stdin.unwrap_or_default()),
            _ => self.run_generic(),
        }
    }

    fn open_editor(self, stdin: String) -> String {
        //TODO find better location
        let file_path = "duk.txt";

        let mut file = File::create(file_path).unwrap();
        file.write_all(stdin.as_bytes()).unwrap();
        drop(file);

        let _ = Command::new("vim")
            .arg(file_path)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        let modified_file = fs::read_to_string(file_path).unwrap();
        println!("{:?}", modified_file);

        fs::remove_file(file_path).unwrap();

        modified_file
    }

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
