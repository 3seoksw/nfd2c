use clap::{Parser, ArgGroup};

#[derive(Parser)]
#[command(
    name = "nfd2c",
    version = "0.1.0",
    author = "WooSeok Kim, 3suksw@gmail.com",
    about = "Converts NFD-formatted file names to NFC format"
)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["directory"]),
        // .args(&["files", "directory"]),
))]
pub struct Args {
    // #[arg(
    //     short = 'f',
    //     long = "files",
    //     value_name = "FILE(S)",
    //     num_args = 1..,
    //     default_value = "./",
    //     conflicts_with = "directory",
    //     help = "File(s) to process",
    // )]
    // pub files: Vec<String>,

    #[arg(
        short = 'd',
        long = "directory",
        value_name = "DIR",
        default_value = "./",
        conflicts_with = "files",
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
