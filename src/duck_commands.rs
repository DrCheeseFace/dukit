use crate::{duck_branch, duck_status, duck_add};

pub enum DuckCommands {
    Status,
    Branch,
    Add,
}

impl DuckCommands {
    pub fn run(&self) {
        match self {
            DuckCommands::Status => duck_status(),
            DuckCommands::Branch => duck_branch(),
            DuckCommands::Add => duck_add(),
        }
    }
}
