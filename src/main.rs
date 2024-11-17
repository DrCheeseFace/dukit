use clap::Parser;
use dukit::duck_commands::DuckCommands;
use std::str;

/// git for ducks
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// file info
    #[arg(short, long, default_value_t = true)]
    status: bool,

    /// branch info
    #[arg(short, long, default_value_t = false)]
    branch: bool,

    /// interactive add
    #[arg(short, long, default_value_t = false)]
    iadd: bool,
}

fn main() {
    let args = Args::parse();
    if args.branch {
        let _ = DuckCommands::Branch.run();
    } else if args.iadd {
        DuckCommands::Add.run();
    } else if args.status {
        DuckCommands::Status.run();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sanity_check() {
        assert_eq!(true, true);
    }
}
