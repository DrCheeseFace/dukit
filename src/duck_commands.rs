use crate::{
    base_commands::BaseCliCommands, errors::DuckErrors, COMMENT_CHAR, CURRENT_BRANCH_CHAR, DELETED_CHAR, DELETED_LABEL, EMPTY_CHAR, INTERACTIVE_ADD_HELP, LINE_SEPERATOR, MODIFIED_CHAR, MODIFIED_LABEL, NOTHING_TO_COMMIT_MESSAGE, NO_FILES_SELECTED_TO_ADD, NO_REMOTE_INFO, RUNNING_GIT_ADD, STAGED_LABEL, TICKED_BOX, UNSTAGED_LABEL, UNTRACKED_CHAR, UNTRACKED_LABEL
};
use termion::color;

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
            DuckCommands::Branch => self.duck_branch(),
            DuckCommands::Add => self.duck_interactive_add(),
        }
    }

    /// pretty git status for files output
    fn duck_file_status(&self) {
        let out = match BaseCliCommands::Status.run(None) {
            Ok(output) => output,
            Err(e) => {
                e.printout();
                return;
            }
        };
        let mut v: Vec<&str> = out.split('\n').collect();

        // find a better way
        v.pop();
        v.pop();

        let mut a: Vec<&str> = Vec::new();
        let mut m: Vec<&str> = Vec::new();
        let mut u: Vec<&str> = Vec::new();
        let mut d: Vec<&str> = Vec::new();

        for line in &v {
            let split: (&str, &str) = line.split_at(2);
            let file = split.1;
            let state = split.0;

            if state.chars().nth(0).unwrap() != EMPTY_CHAR
                && state.chars().nth(0).unwrap() != UNTRACKED_CHAR
            {
                a.push(file)
            }
            if state.chars().nth(1).unwrap() == MODIFIED_CHAR {
                m.push(file)
            }
            if state.chars().nth(1).unwrap() == UNTRACKED_CHAR {
                u.push(file)
            }
            if state.chars().nth(1).unwrap() == DELETED_CHAR {
                d.push(file)
            }
        }

        if v.is_empty() {
            println!(
                "{}\n {}",
                color::Fg(color::Green),
                NOTHING_TO_COMMIT_MESSAGE
            );
            return;
        }

        if !a.is_empty() {
            println!("{}\n {}", color::Fg(color::Green), STAGED_LABEL);
            for i in a {
                println!("{}{}", color::Fg(color::LightGreen), i);
            }
        }
        if !m.is_empty() {
            println!("{}\n {}", color::Fg(color::Yellow), MODIFIED_LABEL);
            for i in m {
                println!("{}{}", color::Fg(color::LightYellow), i);
            }
        }
        if !u.is_empty() {
            println!("{}\n {}", color::Fg(color::Red), UNTRACKED_LABEL);
            for i in u {
                println!("{}{}", color::Fg(color::LightRed), i);
            }
        }
        if !d.is_empty() {
            println!("{}\n {}", color::Fg(color::Red), DELETED_LABEL);
            for i in d {
                println!("{}{}", color::Fg(color::LightRed), i);
            }
        }
    }

    /// pretty git status output for branch info
    fn duck_branch(&self) {
        let cmdout = match BaseCliCommands::RemoteBranch.run(None) {
            Ok(output) => output,
            Err(e) => {
                e.printout();
                return;
            }
        };
        if !cmdout.trim().is_empty() {
            println!("\n{}{}\n", color::Fg(color::Cyan), cmdout.trim());
        } else {
            println!("\n{}{}\n", color::Fg(color::Cyan), NO_REMOTE_INFO);
        }
        let cmdout = match BaseCliCommands::BranchList.run(None) {
            Ok(output) => output,
            Err(e) => {
                e.printout();
                return;
            }
        };

        let branches: Vec<&str> = cmdout.trim().split('\n').collect();
        for branch in &branches {
            if branch.chars().nth(0).unwrap() == CURRENT_BRANCH_CHAR {
                let branch = match branch.strip_prefix(CURRENT_BRANCH_CHAR) {
                    Some(output) => output.trim(),
                    None => {
                        DuckErrors::Fuck.printout();
                        return;
                    }
                };
                println!("{}{}", color::Fg(color::Green), branch);
            } else {
                println!("{}{}", color::Fg(color::Yellow), branch);
            }
        }
    }

    fn duck_interactive_add(&self) {
        let out = match BaseCliCommands::Status.run(None) {
            Ok(output) => output,
            Err(e) => {
                e.printout();
                return;
            }
        };
        let mut v: Vec<&str> = out.split('\n').collect();

        // find a better way
        v.pop();
        v.pop();

        let mut staged: Vec<&str> = Vec::new();
        let mut unstaged: Vec<&str> = Vec::new();

        for line in v {
            let split: (&str, &str) = line.split_at(2);
            let file = split.1;
            let state = split.0;

            if state.chars().nth(0).unwrap() != EMPTY_CHAR
                && state.chars().nth(0).unwrap() != UNTRACKED_CHAR
            {
                staged.push(file)
            } else {
                unstaged.push(file)
            }
        }

        let mut stdin = String::new();
        if !staged.is_empty() {
            stdin.push_str(&format!("{} {}\n", COMMENT_CHAR, STAGED_LABEL));
            for i in staged {
                stdin.push_str(i);
                stdin.push('\n');
            }
            stdin.push('\n');
        }
        stdin.push_str(&format!("{} {}\n", COMMENT_CHAR, LINE_SEPERATOR));
        stdin.push_str("\n\n");

        if !unstaged.is_empty() {
            stdin.push_str(&format!("{} {}\n", COMMENT_CHAR, UNSTAGED_LABEL));
            for i in &unstaged {
                stdin.push_str("[ ]");
                stdin.push_str(i);
                stdin.push('\n');
            }
        }
        stdin.push('\n');
        stdin.push_str(INTERACTIVE_ADD_HELP);

        let textout = match BaseCliCommands::OpenEditor.run(Some(stdin)) {
            Ok(output) => output,
            Err(e) => {
                e.printout();
                return;
            }
        };

        let lines: Vec<&str> = textout.split('\n').collect();
        let line_seperator_index = match lines.iter().position(|&s| s.contains(LINE_SEPERATOR)) {
            Some(output) => output,
            None => {
                DuckErrors::NoMatchingLineSeperatorFound.printout();
                return;
            }
        };

        let mut to_be_added: Vec<&str> = Vec::new();
        for line in &lines[line_seperator_index + 1..lines.len()] {
            if !line.contains(COMMENT_CHAR) && line.contains(TICKED_BOX) {
                let line = match line.strip_prefix(TICKED_BOX) {
                    Some(output) => output,
                    None => {
                        DuckErrors::Fuck.printout();
                        return;
                    }
                };

                to_be_added.push(line)
            }
        }

        if to_be_added.is_empty() {
            println!("{}\n{}", color::Fg(color::Green), NO_FILES_SELECTED_TO_ADD);
            return
        }

        for file in to_be_added {
            println!(
                "{}\n {} {} ",
                color::Fg(color::Yellow),
                RUNNING_GIT_ADD,
                file
            );
            match BaseCliCommands::AddFile.run(Some(file.to_string())) {
                Ok(output) => output,
                Err(e) => {
                    e.printout();
                    return;
                }
            };
            println!("{}{} {}", color::Fg(color::Green), file, STAGED_LABEL);
        }
    }
}
