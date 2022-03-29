use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{self, BufReader, Write};

use thalweg::generator::ThalwegGenerator;
use thalweg::read;

use clap::Parser;

/// Generate a thalweg of an inlet
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Directory containing NONNA-10 bathymetry data
    #[clap(short, long)]
    data: OsString,

    /// File containing the beginning and end of the inlet
    #[clap(short, long)]
    corners: OsString,

    /// File to write resulting path to
    #[clap(short, long, default_value = "path.txt")]
    output: OsString,

    /// Resolution of desired thalweg
    #[clap(short, long, default_value_t = 20)]
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
    let only_positive = data.into_iter().filter(|bath| bath.depth() > 0.0).collect();
    // read corners from corners
    let corners = {
        println!("reading {:?}", args.corners);
        let corners = File::open(args.corners)?;
        let mut reader = BufReader::new(corners);
        read::read_corner_lines(&mut reader)?
    };
    println!("corners: {:?}", corners);
    // set up data
    let generator = ThalwegGenerator::from_points(only_positive, args.resolution);

    // run search
    if let Some(path) = generator.thalweg(corners[0], corners[1]) {
        // got a path
        println!("path contians {} points", path.len());
        let mut file = File::create(args.output)?;
        writeln!(file, "\"Lat (DMS)\" \"Long (DMS)\" \"Depth (m)\"")?;
        for point in path {
            writeln!(file, "{}", point)?;
        }
    } else {
        eprintln!("No path found");
    }

    Ok(())
}
