use std::{fs, path::PathBuf};

use clap::Parser;
use zerocopy::IntoBytes;
use zx297520v3_loader::{
    STAGE1_BASE,
    err::{Error, Result},
    header::Header,
};

#[derive(Parser)]
struct Cli {
    /// Path to input file
    #[arg(short, long)]
    input: PathBuf,

    /// Path to output file
    #[arg(short, long)]
    output: PathBuf,

    /// Path to header file with non-defaults
    #[arg(short, long)]
    preset: Option<PathBuf>,
}

fn entry() -> Result<()> {
    let cli = Cli::parse();
    let mut src = fs::read(cli.input)?;

    if Header::try_read(&src[..size_of::<Header>()]).is_ok() {
        eprintln!("Header already exists");
    } else {
        let header = if let Some(header) = cli.preset {
            let mut header: Header = ron::from_str(&fs::read_to_string(header)?).map_err(|e| Error::InvalidHeaderFormat(e))?;
            header.data_size = src.len() as u32;
            header
        } else {
            let mut header = Header::default();
            header.entry = STAGE1_BASE | 1;
            header.data_size = src.len() as u32;
            header
        };

        let size = size_of::<Header>();
        src.reserve(size);
        header.write_to(&mut src[..size]).map_err(|_| Error::Zerocopy)?;

        println!("{header}");

        fs::write(cli.output, src)?;
    }

    Ok(())
}

fn main() -> core::result::Result<(), String> {
    if let Err(e) = entry() { Err(e.to_string()) } else { Ok(()) }
}
