use one_brc::processor::process_data;
use std::path;

pub fn main() {
    let start = std::time::Instant::now();

    // Read filename from command line arguments:
    let args: Vec<String> = std::env::args().collect();
    let filename = &args.get(1).expect("no filename provided");

    let input_parent = path::Path::new(filename).parent().unwrap();
    let output_filename = input_parent.join("output.csv");

    let lines_processed = process_data(filename, output_filename.to_str().unwrap()).unwrap();

    let duration = start.elapsed();

    println!(
        "Processed {}M lines in {} ms",
        lines_processed / 1_000_000,
        duration.as_millis()
    );

    println!(
        "Expected seconds for 1 billion rows instead {}",
        (duration.as_secs_f64() / lines_processed as f64) * 1_000_000_000_f64
    );
}
