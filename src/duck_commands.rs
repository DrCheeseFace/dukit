use termion::color;

use crate::base_commands::BaseCliCommands;

pub enum DuckCommands {
    Status,
    Branch,
    Add,
}

impl DuckCommands {
    /// DUCKS!
    pub fn run(&self) {
        match self {
            DuckCommands::Status => self.duck_file_status(),
            DuckCommands::Branch => self.duck_current_branch(),
            DuckCommands::Add => self.duck_interactive_add(),
        }
    }

    /// pretty git status for files output
    fn duck_file_status(&self) {
        let out = BaseCliCommands::Status.run(None);
        let mut v: Vec<&str> = out.split('\n').collect();

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

    /// pretty git status output for current branch info
    fn duck_current_branch(&self) {
        let mut out = String::new();
        let cmdout = BaseCliCommands::RemoteBranch.run(None);
        out.push_str(&cmdout.to_string().trim());
        out.push_str("\n");
        let cmdout = BaseCliCommands::CurrentBranch.run(None);
        out.push_str(&cmdout.to_string());

        println!("\n{}{}", color::Fg(color::Magenta), out.trim().to_string())
    }

    fn duck_interactive_add(&self) {
        let cmdout = BaseCliCommands::OpenEditor.run(Some("is this working".to_string()));
        println!("{}", cmdout);
    }
}
