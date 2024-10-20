use clap::Parser;
use pmgr::commands::{Cli, Commands, Command};

#[cfg(debug_assertions)]
const FILE_NAME: &str = ".debug.pmgr";
#[cfg(not(debug_assertions))]
const FILE_NAME: &str = ".pmgr";

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check(args) => args.run(FILE_NAME),
        Commands::Read(args) => args.run(FILE_NAME),
        Commands::List(args) => args.run(FILE_NAME),
        Commands::Init(args) => args.run(FILE_NAME),
        Commands::Create(args) => args.run(FILE_NAME),
        Commands::Delete(args) => args.run(FILE_NAME),
        Commands::Watch(args) => args.run(FILE_NAME),
        Commands::Unwatch(args) => args.run(FILE_NAME),
        Commands::Add(args) => args.run(FILE_NAME),
        Commands::Remove(args) => args.run(FILE_NAME),
        Commands::Task(args) => args.run(FILE_NAME),
    }
}
