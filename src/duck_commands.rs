use crate::{
    base_commands::BaseCliCommands, errors::DuckErrors, COMMENT_CHAR, DELETED_CHAR, DELETED_LABEL,
    EMPTY_CHAR, INTERACTIVE_ADD_HELP, LINE_SEPERATOR, MODIFIED_CHAR, MODIFIED_LABEL,
    NOTHING_TO_COMMIT_MESSAGE, RUNNING_GIT_ADD, STAGED_LABEL, TICKED_BOX, UNSTAGED_LABEL,
    UNTRACKED_CHAR, UNTRACKED_LABEL,
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
            DuckCommands::Branch => self.duck_current_branch(),
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

    /// pretty git status output for current branch info
    fn duck_current_branch(&self) {
        let mut out = String::new();
        let cmdout = match BaseCliCommands::RemoteBranch.run(None) {
            Ok(output) => output,
            Err(e) => {
                e.printout();
                return;
            }
        };
        out.push_str(cmdout.to_string().trim());
        out.push('\n');
        let cmdout = match BaseCliCommands::CurrentBranch.run(None) {
            Ok(output) => output,
            Err(e) => {
                e.printout();
                return;
            }
        };
        out.push_str(&cmdout.to_string());

        println!("\n{}{}", color::Fg(color::Magenta), out.trim());
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
            stdin.push_str(&format!("# {}\n", STAGED_LABEL));
            for i in staged {
                stdin.push_str(i);
                stdin.push('\n');
            }
        }

        stdin.push('\n');
        stdin.push_str(LINE_SEPERATOR);

        if !unstaged.is_empty() {
            stdin.push_str(&format!("\n\n# {}\n", UNSTAGED_LABEL));
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
        let line_seperator_index = match lines.iter().position(|&s| s == LINE_SEPERATOR) {
            Some(output) => output,
            None => {
                DuckErrors::TODO.printout();
                return;
            }
        };

        let mut to_be_added: Vec<&str> = Vec::new();
        for line in &lines[line_seperator_index + 1..lines.len()] {
            if !line.contains(COMMENT_CHAR) && line.contains(TICKED_BOX) {
                let line = match line.strip_prefix(TICKED_BOX) {
                    Some(output) => output,
                    None => {
                        DuckErrors::TODO.printout();
                        return;
                    }
                };

                to_be_added.push(line)
            }
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
