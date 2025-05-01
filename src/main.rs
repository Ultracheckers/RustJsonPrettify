use anyhow::{Context, Result};
use clap::Parser;
use json_prettify::{prettify_json, read_json_from_file, read_json_from_stdin};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: Option<PathBuf>,

    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(short, long, default_value_t = 2)]
    indent: u8,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let content = if let Some(input_path) = &cli.input {
        read_json_from_file(input_path)?
    } else {
        read_json_from_stdin()?
    };

    let pretty_json = prettify_json(&content)?;

    if let Some(output_path) = &cli.output {
        std::fs::write(output_path, pretty_json)
            .with_context(|| format!("Failed to write to file: {}", output_path.display()))?;
    } else {
        println!("{}", pretty_json);
    }

    Ok(())
}