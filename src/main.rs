extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;

fn main() {
    // Check if the correct number of arguments is provided
    if args().len() != 3 {
        eprintln!("Usage: <source> <target>");
        return;
    }

    // Get the source and target file paths from command line arguments
    let source = args().nth(1).expect("Missing source file argument");
    let target = args().nth(2).expect("Missing target file argument");

    // Open the source file
    let input_file = File::open(&source).expect("Failed to open source file");
    let mut input = BufReader::new(input_file);

    // Create the target file
    let output_file = File::create(&target).expect("Failed to create target file");
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    // Start timing
    let start = Instant::now();

    // Copy data from the source to the compressed output
    copy(&mut input, &mut encoder).expect("Failed to copy data");

    // Finish the compression process
    let output_file = encoder.finish().expect("Failed to finish compression");

    // Print results
    println!(
        "Source length: {:?}",
        input.get_ref().metadata().expect("Failed to get metadata").len()
    );
    println!(
        "Target length: {:?}",
        output_file.metadata().expect("Failed to get metadata").len()
    );
    println!("Elapsed time: {:?}", start.elapsed());
}
