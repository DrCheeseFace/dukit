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

    /// fuzzy switch branch
    #[arg(short, long, default_value_t = false)]
    fuzzybranch: bool,

    /// log info
    #[arg(short, long, default_value_t = false)]
    log: bool,

    /// interactive add via keys (for when you have few files modified)
    #[arg(short, long, default_value_t = false)]
    key_add: bool,
}

fn main() {
    let args = Args::parse();
    if args.branch {
        DuckCommands::Branch.run();
    } else if args.iadd {
        DuckCommands::Add.run();
    } else if args.fuzzybranch {
        DuckCommands::FuzzyBranchSwitch.run();
    } else if args.key_add {
        DuckCommands::KeyAdd.run();
    } else if args.log {
        DuckCommands::Log.run();
    } else if args.status {
        DuckCommands::Status.run();
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn sanity_check() {
        assert_eq!(true, true);
    }
}
