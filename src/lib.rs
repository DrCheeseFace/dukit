use termion::color;
pub mod base_commands;
pub mod duck_commands;
use crate::base_commands::BaseCliCommands;

use std::process::Output;

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

pub fn duck_status() {
    let out = BaseCliCommands::Status.run();
    let binding = out.to_string();
    let mut v: Vec<&str> = binding.split('\n').collect();

    // find a better way
    v.pop();
    v.pop();

    let mut a: Vec<&str> = Vec::new();
    let mut m: Vec<&str> = Vec::new();
    let mut u: Vec<&str> = Vec::new();
    let mut d: Vec<&str> = Vec::new();

    for line in v {
        let split: (&str, &str) = line.split_at(2);
        let file = split.1;
        let state = split.0;

        if state.chars().nth(0).unwrap() != ' ' && state.chars().nth(0).unwrap() != '?' {
            a.push(file)
        }
        if state.chars().nth(1).unwrap() == 'M' {
            m.push(file)
        }
        if state.chars().nth(1).unwrap() == '?' {
            u.push(file)
        }
        if state.chars().nth(1).unwrap() == 'D' {
            d.push(file)
        }
    }

    if !a.is_empty() {
        println!("{}\n S", color::Fg(color::Green));
        for i in a {
            println!("{}{}", color::Fg(color::LightGreen), i);
        }
    }
    if !m.is_empty() {
        println!("{}\n M", color::Fg(color::Yellow));
        for i in m {
            println!("{}{}", color::Fg(color::LightYellow), i);
        }
    }
    if !u.is_empty() {
        println!("{}\n U", color::Fg(color::Red));
        for i in u {
            println!("{}{}", color::Fg(color::LightRed), i);
        }
    }
    if !d.is_empty() {
        println!("{}\n D", color::Fg(color::Red));
        for i in d {
            println!("{}{}", color::Fg(color::LightRed), i);
        }
    }
}

pub fn duck_branch() {
    let mut out = String::new();
    let cmdout = BaseCliCommands::RemoteBranch.run();
    out.push_str(&cmdout.to_string());
    let cmdout = BaseCliCommands::CurrentBranch.run();
    out.push_str(&cmdout.to_string());

    println!("\n{}{}", color::Fg(color::LightRed), out.trim().to_string())
}
