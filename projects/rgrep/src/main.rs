use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::ExitCode;

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
    #[arg()]
    file_name: PathBuf,
}

enum RunOutput {
    FoundLine,
    NotFound,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(RunOutput::FoundLine) => ExitCode::SUCCESS,
        Ok(RunOutput::NotFound) => ExitCode::from(1),
        Err(error) => {
            eprintln!("rgrep error: {error}");
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<RunOutput, Box<dyn std::error::Error>> {
    let file = File::open(cli.file_name)?;
    let buffer = BufReader::new(file);
    let result = search(buffer, &cli.pattern)?;
    if result.is_empty() {
        return Ok(RunOutput::NotFound);
    }
    for line in result {
        println!("{line}")
    }
    Ok(RunOutput::FoundLine)
}
