use std::error::Error;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

use thalweg::bathymetry::{Bathymetry, Point};
use thalweg::format::{self, OutputFormat};
use thalweg::generator::ThalwegGenerator;
use thalweg::read;

use clap::{Args, Parser, Subcommand};

/// Generate a thalweg of an inlet
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generates a new thalweg
    Generate(GenerateArgs),

    /// Apply a path to bathymetry, effectively creating a thalweg
    FromPath(FromPathArgs),
}

// Arguments for generate
#[derive(Args, Debug)]
struct GenerateArgs {
    /// Number of times to apply improvement step
    #[clap(long, default_value_t = 1)]
    rounds: usize,

    /// Whether or not to simplify the generated thalweg
    #[clap(long)]
    simplify: bool,

    /// Skip adding resolution to final thalweg
    #[clap(long)]
    sparse: bool,

    /// Attempt a best first guess by using depths in bathymetry as weights
    #[clap(short, long)]
    weighted: bool,

    #[clap(flatten)]
    common: CommonArgs,
}

// Arguments for from-path
#[derive(Args, Debug)]
struct FromPathArgs {
    #[clap(flatten)]
    common: CommonArgs,
}

// common arguments
#[derive(Args, Debug, Clone)]
struct CommonArgs {
    /// File containing the relevant points along the inlet
    points: OsString,

    /// Directory containing NONNA-10 bathymetry data
    data: OsString,

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
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let (path, args) = match &cli.command {
        Commands::Generate(args) => {
            // points represents points of interest along the inlet
            let data = read_bathymetry_data(&args.common.data)?;
            let points = read_corner_data(&args.common.points)?;
            let generator = ThalwegGenerator::new(data, args.common.resolution, args.weighted);
            let mut full_path = vec![];
            for ends in points.windows(2) {
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
            println!("path contains {} points", full_path.len());
            for _ in 0..args.rounds {
                full_path = improve(&full_path, &generator, args.simplify);
            }
            let path = if !args.sparse {
                println!("Increasing density of path");
                generator.populate(&full_path)
            } else {
                full_path
            };
            (path, args.common.clone())
        }
        Commands::FromPath(args) => {
            // points represents a full path along the inlet
            let data = read_bathymetry_data(&args.common.data)?;
            let points = read_corner_data(&args.common.points)?;
            let generator = ThalwegGenerator::new(data, args.common.resolution, false);
            let path = generator.from_path(&points);
            (generator.populate(&path), args.common.clone())
        }
    };

    let path_vec = format::convert(args.format, &path);

    let output_path = PathBuf::from(args.prefix);
    let output_file = output_path
        .join("path.txt")
        .with_extension(format::extension(args.format));

    let mut file = File::create(output_file)?;
    file.write_all(path_vec.as_bytes())?;

    Ok(())
}

fn read_bathymetry_data<T: AsRef<Path>>(dir: &T) -> Result<Vec<Bathymetry>, Box<dyn Error>> {
    let mut data = vec![];
    for entry in fs::read_dir(dir)? {
        let file_name = entry?.path();
        let file = File::open(file_name)?;
        let mut reader = BufReader::new(file);
        data.extend(read::bathymetry::from_nonna(&mut reader)?);
    }
    Ok(data.into_iter().filter(|bath| bath.depth() > 0.0).collect())
}

fn read_corner_data<T: AsRef<Path>>(file: &T) -> Result<Vec<Point>, Box<dyn Error>> {
    let points = File::open(file)?;
    let mut reader = BufReader::new(points);
    if let Some(ext) = file.as_ref().extension() {
        match ext.to_str() {
            Some("txt") => read::point::from_nonna(&mut reader),
            Some("csv") => read::point::from_csv(&mut reader),
            Some(..) => Ok(vec![]),
            None => Ok(vec![]),
        }
    } else {
        read::point::from_nonna(&mut reader)
    }
}

fn improve(path: &[Bathymetry], generator: &ThalwegGenerator, simplify: bool) -> Vec<Bathymetry> {
    let mut current_path = generator.add_midpoints(path);
    loop {
        // find fixed-point thalweg - mostly in an attempt to ensure the thalweg does not pass over land
        let new_path = generator.sink(&current_path);
        if new_path == current_path {
            break if simplify {
                generator.simplify(&current_path)
            } else {
                current_path
            };
        }
        // combine points that are too close and may produce strange paths on further sink steps
        current_path = generator.shrink(&new_path);
    }
}
