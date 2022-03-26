use std::fs::{self, File};
use std::io::{self, BufReader};

use thalweg::read;

use clap::Parser;

/// Generate a thalweg of an inlet
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Directory containing NONNA-10 bathymetry data
    #[clap(short, long)]
    data: String,

    /// File containing the beginning and end of the inlet
    #[clap(short, long)]
    corners: String,

    /// Resolution of desired thalweg
    #[clap(short, long, default_value_t = 10)]
    resolution: usize,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // read data from dir
    let mut data = vec![];
    for entry in fs::read_dir(args.data)? {
        let file_name = entry?.path();
        println!("reading {:?}", file_name);
        let file = File::open(file_name)?;
        let mut reader = BufReader::new(file);
        data.extend(read::read_data_lines(&mut reader)?);
    }
    println!("{} data values", data.len());
    // read corners from corners
    let corners = {
        println!("reading {}", args.corners);
        let corners = File::open(args.corners)?;
        let mut reader = BufReader::new(corners);
        read::read_corner_lines(&mut reader)?
    };
    println!("corners: {:?}", corners);
    // set up data
    // run search

    Ok(())
}
