use termion::color;

//TODO return the correct thangs
#[derive(Debug)]
pub enum DuckErrors {
    TODO,
}

impl std::fmt::Display for DuckErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TODO => write!(f, "ruh roh raggy!"),
        }
    }
}

impl DuckErrors {
    pub fn printout(&self) {
        eprintln!(
            "{} Error while running 'duck command': {}",
            color::Fg(color::Red),
            self
        );
    }
}
