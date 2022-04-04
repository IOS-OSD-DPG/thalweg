use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

use thalweg::read;
use thalweg::section;

use clap::Parser;

// Generate section info from a thalweg
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// File containing thalweg data
    thalweg: OsString,

    /// Output file
    #[clap(short, long, default_value = "section.csv")]
    output: OsString,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let thalweg_file = PathBuf::from(args.thalweg);
    let section_file = PathBuf::from(args.output);

    let path = {
        let file = File::open(&thalweg_file)?;
        let mut reader = BufReader::new(file);
        if let Some(ext) = thalweg_file.extension() {
            match ext.to_str() {
                Some("txt") => read::thalweg::from_nonna(&mut reader)?,
                Some("csv") => read::thalweg::from_csv(&mut reader)?,
                Some("geojson") => read::thalweg::from_geojson(&mut reader)?,
                Some(..) => vec![],
                None => vec![],
            }
        } else {
            read::thalweg::from_nonna(&mut reader)?
        }
    };

    let section_vec = section::section(&path);

    let mut file = File::create(section_file)?;
    file.write_all(section::to_csv(&section_vec).as_bytes())?;

    Ok(())
}
