pub mod base_commands;
pub mod duck_commands;
use crate::base_commands::BaseCliCommands;

use std::{ops::Index, process::Output};

#[derive(Debug)]
pub struct DisplayableCliCommand(Output);

impl std::fmt::Display for DisplayableCliCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stdout = String::from_utf8_lossy(&self.0.stdout);
        let stderr = String::from_utf8_lossy(&self.0.stderr);

        if stderr.is_empty() {
            return writeln!(f, "{}", stdout.trim());
        }
        return writeln!(f, "{}", stderr.trim());
    }
}

pub fn tempprintout(printouts: Vec<String>) {
    for i in printouts {
        println!("{}", i)
    }
}

pub fn duck_status() -> String {
    let out = BaseCliCommands::Status.run();
    let binding = out.to_string();
    let mut v: Vec<&str> = binding.split('\n').collect();
    v.pop();

    let mut a: Vec<&str> = Vec::new();
    let mut m: Vec<&str> = Vec::new();
    let mut u: Vec<&str> = Vec::new();

    for lines in v {
        let split: Vec<&str> = lines.split_whitespace().collect();
        let file = *split.get(1).unwrap();
        let state = *split.get(0).unwrap();

        if state.contains('A') {
            a.push(file)
        }
        if state.contains('M') {
            m.push(file)
        }
        if state.contains('?') {
            u.push(file)
        }
    }

    let mut out = String::new();
    out.push_str("\nA \n");
    for i in a {
        out.push_str(i);
        out.push_str("\n");
    }
    out.push_str("\nM \n");
    for i in m {
        out.push_str(i);
        out.push_str("\n");
    }
    out.push_str("\nU \n");
    for i in u {
        out.push_str(i);
        out.push_str("\n");
    }

    out
}

pub fn duck_branch() -> String {
    let mut out = String::new();
    let cmdout = BaseCliCommands::CurrentBranch.run();
    out.push_str(&cmdout.to_string());
    let cmdout = BaseCliCommands::RemoteBranch.run();
    out.push_str(&cmdout.to_string());

    out.trim().to_string()
}
