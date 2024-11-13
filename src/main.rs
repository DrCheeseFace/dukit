use clap::Parser;
use duk::duck_commands::DuckCommands;
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
    let mut out: String = String::new();
    if args.status {
        out = DuckCommands::Status.run()
    }
    if args.branch {
        out = DuckCommands::Branch.run()
    }

    println!("{}", out)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sanity_check() {
        assert_eq!(true, true);
    }
}
