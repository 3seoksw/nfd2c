use clap::{Parser, ArgGroup};

#[derive(Parser)]
#[command(
    name = "nfd2c",
    version = "0.3.0",
    author = "WooSeok Kim, 3suksw@gmail.com",
    about = "Converts NFD-formatted file names to NFC format"
)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["directory"]),
))]
pub struct Args {
    #[arg(
        short = 'd',
        long = "directory",
        value_name = "DIR",
        default_value = "./",
        index = 1,
        help = "Directory to process files inside",
    )]
    pub directory: Option<String>,

    #[arg(
        short = 'r',
        long = "recursive",
        default_value_t = false,
        help = "Recursively process files in subdirectories",
    )]
    pub recursive: bool,

    #[arg(
        short = 'v',
        long = "verbose",
        action = clap::ArgAction::SetTrue,
        default_value_t = false,
        help = "Show more detailed output",
    )]
    pub verbose: bool,
}
