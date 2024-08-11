use std::io::{BufWriter, Write};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

// u32::MAX is 4.29B so u32 should be enough for 1 billion
struct Stats {
    sum: f32,
    count: u32,
    min: f32,
    max: f32,
}

impl Stats {
    fn new() -> Self {
        Self {
            sum: 0.0,
            count: 0,
            min: f32::MAX,
            max: f32::MIN,
        }
    }
}

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

    let mut stats_map: HashMap<String, Stats> = HashMap::new();

    let mut lines_processed: u32 = 0;

    for line in reader.lines() {
        let line = line.expect("cannot read line");
        if line.starts_with('#') {
            continue;
        }

        let mut splitted_line = line.split(';');

        let station = splitted_line.next().unwrap().to_string();
        let number = splitted_line.next().unwrap().parse::<f32>().unwrap();

        let current_stats = stats_map.entry(station).or_insert(Stats::new());

        if number < current_stats.min {
            current_stats.min = number;
        }

        if number > current_stats.max {
            current_stats.max = number;
        }

        current_stats.sum += number;
        current_stats.count += 1;

        lines_processed += 1;

        // println!("station: {}, number: {}", station, number);
    }

    for (station, stats) in stats_map.into_iter() {
        // println!("{};{};{};{}", station, min, (sum / (count as f32)), max);
        writer.write_all(
            format!(
                "{};{};{};{}\n",
                station,
                stats.min,
                (stats.sum / (stats.count as f32)),
                stats.max
            )
            .as_bytes(),
        )?;
    }

    Ok(lines_processed)
}
