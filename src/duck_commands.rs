use crate::{duck_branch, duck_status};

pub enum DuckCommands {
    Status,
    Branch,
}

impl DuckCommands {
    pub fn run(&self) -> String {
        match self {
            DuckCommands::Status => duck_status(),
            DuckCommands::Branch => duck_branch(),
        }
    }
}
