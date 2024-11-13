use std::process::Command;

use crate::DisplayableCliCommand;

pub enum BaseCliCommands {
    Status,
    CurrentBranch,
    RemoteBranch,
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
            BaseCliCommands::Todo => "echo 'hi :3'",
        }
    }

    pub fn run(self) -> DisplayableCliCommand {
        let output = Command::new("sh")
            .arg("-c")
            .arg(self.get_cli_command())
            .output()
            .expect("failed to execute process");
        let displayable_output = DisplayableCliCommand(output);
        return displayable_output;
    }
}
