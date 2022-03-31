use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::path::PathBuf;

use thalweg::format::{self, OutputFormat};
use thalweg::generator::ThalwegGenerator;
use thalweg::read;
use thalweg::section;

use clap::Parser;

/// Generate a thalweg of an inlet
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Directory containing NONNA-10 bathymetry data
    #[clap(short, long)]
    data: OsString,

    /// File containing the beginning and end of the inlet
    #[clap(short, long)]
    corners: OsString,

    /// Directory to write resulting path to
    #[clap(short, long, default_value = ".")]
    prefix: OsString,

    /// Format of output file.
    /// Has no effect on section info output
    #[clap(short, long, default_value_t = OutputFormat::default())]
    format: OutputFormat,

    /// Resolution of desired thalweg
    #[clap(short, long, default_value_t = 20)]
    resolution: usize,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut data = vec![];
    for entry in fs::read_dir(args.data)? {
        let file_name = entry?.path();
        println!("reading {:?}", file_name);
        let file = File::open(file_name)?;
        let mut reader = BufReader::new(file);
        data.extend(read::read_data_lines(&mut reader)?);
    }
    println!("{} data values", data.len());
    let data = data.into_iter().filter(|bath| bath.depth() > 0.0).collect();

    let corners = {
        println!("reading {:?}", args.corners);
        let corner_path = PathBuf::from(args.corners);
        let corners = File::open(&corner_path)?;
        let mut reader = BufReader::new(corners);
        if let Some(ext) = corner_path.extension() {
            match ext.to_str() {
                Some("txt") => read::read_corner_lines(&mut reader)?,
                Some("csv") => read::read_corner_csv(&mut reader)?,
                Some(..) => vec![],
                None => vec![],
            }
        } else {
            read::read_corner_lines(&mut reader)?
        }
    };
    println!("corners: {:?}", corners);

    let generator = ThalwegGenerator::from_points(data, args.resolution);
    if let Some(path) = generator.thalweg(
        *corners.first().expect("no source"),
        *corners.last().expect("no sink"),
    ) {
        println!("path contians {} points", path.len());

        let path_vec = format::convert(args.format, &path);
        let section_vec = section::section(&path);

        let output_path = PathBuf::from(args.prefix);
        let output_file = output_path
            .join("path.txt")
            .with_extension(format::extension(args.format));
        let section_file = output_path.join("section.csv");

        let mut file = File::create(output_file)?;
        file.write_all(path_vec.as_bytes())?;

        let mut file = File::create(section_file)?;
        file.write_all(section::to_csv(&section_vec).as_bytes())?;
    } else {
        eprintln!("No path found");
    }

    Ok(())
}
