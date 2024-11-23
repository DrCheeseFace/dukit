use termion::color;

#[derive(Debug)]
pub enum DuckErrors {
    Fuck,
    GitAdd,
    GitGeneric,
    SpawnChildProccesForEditor,
    BadExitCodeForEditor,
    CouldNotWriteToTempFile,
    SpawnChildProccesForGeneric,
    NoMatchingLineSeperatorFound,
    CouldNotReadTempFile,
    LocalChangesOverwrittenByCheckout,
    GitSwitchGeneric,
    NoBranchGiven,
}

impl std::fmt::Display for DuckErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fuck => write!(f, "not sure how you did it but an error occured that i didnt know could be possible. well done!"),
            Self::GitAdd => write!(f, "could not run git add"),
            Self::GitGeneric => write!(f, "could not run git command"),
            Self::SpawnChildProccesForEditor => write!(f, "could not open editor"),
            Self::BadExitCodeForEditor => write!(f, "got bad exit code when closing editor"),
            Self::CouldNotWriteToTempFile => write!(f, "could not write to temp file"),
            Self::SpawnChildProccesForGeneric => write!(f, "could not spawn child process for command"),
            Self::NoMatchingLineSeperatorFound=> write!(f, "could not find matching line seperator. (dont mess with the stuff thats commented)"),
            Self::CouldNotReadTempFile=> write!(f, "could not read temp file"),
            Self::LocalChangesOverwrittenByCheckout=> write!(f, "Your local changes would be overwritten. commit or stash your changes"),
            Self::GitSwitchGeneric=> write!(f, "could not run git switch"),
            Self::NoBranchGiven=> write!(f, "no branch name given")
        }
    }
}

impl DuckErrors {
    pub fn printout(&self) {
        eprintln!("{} \nError: {}", color::Fg(color::Red), self);
    }
}
