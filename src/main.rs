mod args;
mod processor;

use args::Args;
use clap::Parser;
use processor::process_directory;

fn main() {
    let args= Args::parse();

    if let Some(directory) = args.directory {
        process_directory(&directory, args.recursive, args.verbose);
    } else {
        eprintln!("Input not provided.");
    }
}
