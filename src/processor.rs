use std::io::{BufWriter, Write};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn process_data(input_filename: &str, output_filename: &str) -> Result<u32, io::Error> {
    let input_file = File::open(input_filename).unwrap_or_else(|_| {
        let pwd = std::env::current_dir().expect("cannot get current directory");
        panic!(
            "cannot open input file {}. pwd: {}",
            input_filename,
            pwd.to_str().unwrap()
        )
    });
    let reader = BufReader::new(input_file);

    let output_file = File::create(output_filename).expect("cannot create output file");
    let mut writer = BufWriter::new(output_file);

    // u32::MAX is 4.29B so it should be enough for 1BRC

    let mut sum_and_count_map: HashMap<String, (f32, u32)> = HashMap::new();
    let mut max_map: HashMap<String, f32> = HashMap::new();
    let mut min_map: HashMap<String, f32> = HashMap::new();

    let mut lines_processed: u32 = 0;

    for line in reader.lines() {
        let line = line.expect("cannot read line");
        if line.starts_with('#') {
            continue;
        }

        let mut splitted_line = line.split(';');

        let station = splitted_line.next().unwrap().to_string();
        let number = splitted_line.next().unwrap().parse::<f32>().unwrap();

        let current_min = min_map.entry(station.clone()).or_insert(number);
        if number < *current_min {
            *current_min = number;
        }

        let current_max = max_map.entry(station.clone()).or_insert(number);
        if number > *current_max {
            *current_max = number;
        }

        let (current_sum, current_count) = sum_and_count_map
            .entry(station.clone())
            .or_insert((number, 1));

        *current_sum += number;
        *current_count += 1;

        lines_processed += 1;

        // println!("station: {}, number: {}", station, number);
    }

    for (station, (sum, count)) in sum_and_count_map.into_iter() {
        let min = min_map.get(&station).unwrap();
        let max = max_map.get(&station).unwrap();
        // println!("{};{};{};{}", station, min, (sum / (count as f32)), max);
        writer.write_all(
            format!("{};{};{};{}\n", station, min, (sum / (count as f32)), max).as_bytes(),
        )?;
    }

    Ok(lines_processed)
}
