use peroxide::fuga::*;
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // Read the parquet file
    let df = DataFrame::read_parquet(filename)?;

    // Print the DataFrame
    df.print();

    Ok(())
}
