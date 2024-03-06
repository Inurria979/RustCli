use anyhow::{Context, Result};
use clap::Parser;
// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

pub fn nice_error_reporting() -> Result<(), >{

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read that file {}", &args.path.display()))?;
    println!("The file content is: {}", content);
    return Ok(());
}