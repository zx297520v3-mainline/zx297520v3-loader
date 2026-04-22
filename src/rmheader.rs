use std::{fs, path::PathBuf};

use clap::Parser;
use zx297520v3_loader::{err::Result, header::Header};

#[derive(Parser)]
struct Cli {
    /// Path to input file
    #[arg(short, long)]
    input: PathBuf,

    /// Path to output file
    #[arg(short, long)]
    output: PathBuf,

    /// Path to file where header data should be saved
    #[arg(short, long)]
    preset: Option<PathBuf>,
}

fn entry() -> Result<()> {
    let cli = Cli::parse();
    let src = fs::read(cli.input)?;

    match Header::try_read(&src[..size_of::<Header>()]) {
        Ok(header) => {
            fs::write(cli.output, &src[size_of::<Header>()..])?;
            if let Some(preset) = cli.preset {
                fs::write(preset, ron::ser::to_string_pretty(header, ron::ser::PrettyConfig::default())?)?;
            }

            println!("{header}");
        }
        Err(_) => eprintln!("Bad header data"),
    }

    Ok(())
}

fn main() -> core::result::Result<(), String> {
    if let Err(e) = entry() { Err(e.to_string()) } else { Ok(()) }
}
