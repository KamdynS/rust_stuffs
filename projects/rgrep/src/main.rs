use std::fs;
use std::path::PathBuf;

use clap::Parser;

use rgrep::search;

#[derive(Parser)]
#[command(name = "rgrep")]
#[command(version = "1.0")]
#[command(about = "Custom grep", long_about = None)]
struct Cli {
    /// The pattern we match on
    #[arg()]
    pattern: String,

    /// The path to the file to grep
    #[arg(default_value = "")]
    file_name: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let content = fs::read_to_string(cli.file_name)?;
    let result = search(&content, &cli.pattern);
    for line in result {
        println!("{line}")
    }

    Ok(())
}
