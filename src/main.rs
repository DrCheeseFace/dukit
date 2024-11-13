use clap::Parser;
use duk::duck_commands::DuckCommands;
use duk::tempprintout;
use std::str;

/// git for ducks
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// file info
    #[arg(short, long, default_value_t = false)]
    status: bool,

    /// branch info
    #[arg(short, long, default_value_t = false)]
    branch: bool,
}

fn main() {
    let args = Args::parse();
    let mut out: Vec<String> = Vec::new();
    if args.status {
        out.push(DuckCommands::Status.run())
    }
    if args.branch {
        out.push(DuckCommands::Branch.run())
    }

    tempprintout(out)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sanity_check() {
        assert_eq!(true, true);
    }
}
