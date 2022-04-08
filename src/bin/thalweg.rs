use std::error::Error;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::PathBuf;

use thalweg::format::{self, OutputFormat};
use thalweg::generator::ThalwegGenerator;
use thalweg::read;

use clap::Parser;

/// Generate a thalweg of an inlet
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Directory containing NONNA-10 bathymetry data
    data: OsString,

    /// File containing the beginning and end of the inlet
    corners: OsString,

    /// Directory to write resulting path to
    #[clap(short, long, default_value = ".")]
    prefix: OsString,

    /// Format of output file.
    /// Has no effect on section info output
    #[clap(short, long, default_value_t = OutputFormat::default())]
    format: OutputFormat,

    /// Resolution of desired thalweg in metres
    #[clap(short, long, default_value_t = 1000)]
    resolution: usize,

    /// Skip adding resolution to final thalweg
    #[clap(short, long)]
    sparse: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut data = vec![];
    for entry in fs::read_dir(args.data)? {
        let file_name = entry?.path();
        let file = File::open(file_name)?;
        let mut reader = BufReader::new(file);
        data.extend(read::bathymetry::from_nonna(&mut reader)?);
    }
    println!("{} data values", data.len());
    let data = data.into_iter().filter(|bath| bath.depth() > 0.0).collect();

    let corners = {
        let corner_path = PathBuf::from(args.corners);
        let corners = File::open(&corner_path)?;
        let mut reader = BufReader::new(corners);
        if let Some(ext) = corner_path.extension() {
            match ext.to_str() {
                Some("txt") => read::point::from_nonna(&mut reader)?,
                Some("csv") => read::point::from_csv(&mut reader)?,
                Some(..) => vec![],
                None => vec![],
            }
        } else {
            read::point::from_nonna(&mut reader)?
        }
    };

    let mut full_path = vec![];
    let generator = ThalwegGenerator::from_points(data, args.resolution);
    for ends in corners.windows(2) {
        let source = *ends.first().expect("no source");
        let sink = *ends.last().expect("no sink");
        if let Some(mut path) = generator.thalweg(source, sink) {
            full_path.append(&mut path);
        } else {
            return Err(Box::<dyn Error>::from(format!(
                "No path found between {:?} and {:?}",
                source, sink
            )));
        }
    }
    println!("path contians {} points", full_path.len());
    let mut current_path = full_path;
    let path = loop {
        // find fixed-point thalweg - mostly in an attempt to ensure the thalweg does not pass over land
        let new_path = generator.sink(&current_path);
        if new_path == current_path {
            break current_path;
        }
        // combine points that are too close and may produce strange paths on further sink steps
        current_path = generator.shrink(&new_path);
    };

    let path = if !args.sparse {
        println!("Increasing density of path");
        generator.populate(&path)
    } else {
        path
    };

    println!("path now contians {} points", path.len());
    let path_vec = format::convert(args.format, &path);

    let output_path = PathBuf::from(args.prefix);
    let output_file = output_path
        .join("path.txt")
        .with_extension(format::extension(args.format));

    let mut file = File::create(output_file)?;
    file.write_all(path_vec.as_bytes())?;

    Ok(())
}
