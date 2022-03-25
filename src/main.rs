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

fn main() {
    let args = Args::parse();

    // read data from dir
    // read corners from corners
    // set up data
    // run search
}
