use memory_stats::memory_stats;
use rayon::{current_thread_index, prelude::*};
use std::io::{BufWriter, Read, Seek, Write};
use std::os::unix::fs::FileExt;
use std::str::Lines;
use std::time::Instant;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
};

// u32::MAX is 4.29B so u32 should be enough for 1 billion
#[derive(Clone)]
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

pub fn generate_chunks(filename: &str) -> Result<Vec<(usize, usize)>, io::Error> {
    let file = File::open(filename).unwrap_or_else(|_| {
        let pwd = std::env::current_dir().expect("cannot get current directory");
        panic!(
            "cannot open input file {}. pwd: {}",
            filename,
            pwd.to_str().unwrap()
        )
    });

    let start_time = Instant::now();

    let cpu_count: usize = std::thread::available_parallelism().unwrap().into(); // Decides the number of chunks
    let file_size = file.metadata()?.len() as usize;
    let chunk_size = file_size / cpu_count;

    let mut chunks: Vec<(usize, usize)> = vec![];
    let mut start = 0;

    while start < file_size {
        let mut end = (start + chunk_size).min(file_size);

        // Find newline
        let mut last_seen_chars = [0; 1];
        while end > start && last_seen_chars[0] != b'\n' {
            end -= 1;
            file.read_exact_at(&mut last_seen_chars, end as u64)?;
            // Note: we are reading each char at a time, but I believe OS would
            // load a memory page from disk. So it won't be seeking disk for each char.
        }

        chunks.push((start, end));

        // dbg!(end - start);

        start = end + 1; // start from the the latest endpoint next time
    }

    println!(
        "Time taken in generating chunks: {}μs",
        start_time.elapsed().as_micros()
    );

    Ok(chunks)
}

fn process_chunk(lines: Lines, _num_lines: usize) -> HashMap<String, Stats> {
    let mut stats_map: HashMap<String, Stats> = HashMap::new();

    for line in lines {
        if line.starts_with('#') {
            continue;
        }

        let mut splitted_line = line.split(';');

        let station = splitted_line.next().unwrap().to_string();
        let number = splitted_line.next().unwrap().parse::<f32>().unwrap();

        let mut current_stats: Stats = {
            let stats_map = stats_map.get(&station);
            if let Some(current_stats) = stats_map {
                current_stats.clone()
            } else {
                Stats::new()
            }
        };

        if number < current_stats.min {
            current_stats.min = number;
        }

        if number > current_stats.max {
            current_stats.max = number;
        }

        current_stats.sum += number;
        current_stats.count += 1;

        stats_map.insert(station, current_stats);

        // let thread_id = current_thread_index();
        // dbg!(thread_id.unwrap(), num_lines);

        // *lines_processed_guard.lock().unwrap() += 1;
    }

    stats_map
}

pub fn process_data(input_filename: &str, output_filename: &str) -> Result<u32, io::Error> {
    let mut chunks = generate_chunks(input_filename).expect("cannot generate chunks");
    chunks.truncate(2); // To load all the data, you need 13G free RAM
    dbg!(&chunks);

    let workers_time = Instant::now();

    let chunks_stat_maps: Vec<HashMap<String, Stats>> = chunks
        .par_iter()
        .map(|(start, end)| {
            let file = File::open(input_filename).unwrap();
            let mut reader = BufReader::new(file);

            // dbg!(*end - *start);

            reader.seek(io::SeekFrom::Start(*start as u64)).unwrap();
            let mut buffer = Vec::new();
            reader
                .take((*end - *start) as u64)
                .read_to_end(&mut buffer)
                .unwrap();

            // dbg!(current_thread_index().unwrap(), buffer.len());

            if let Some(usage) = memory_stats() {
                println!(
                    "Current physical memory usage: {}MB",
                    usage.physical_mem / 1024 / 1024
                );
                println!(
                    "Current virtual memory usage: {}MB",
                    usage.virtual_mem / 1024 / 1024
                );
            } else {
                println!("Couldn't get the current memory usage :(");
            }

            let content = String::from_utf8_lossy(&buffer);
            let chunk_stats_map = process_chunk(content.lines(), content.len());
            chunk_stats_map
        })
        .collect();

    println!("Time taken by workers {}s", workers_time.elapsed().as_secs());

    // pb.finish_with_message("Done");

    let mut final_stats_map: HashMap<String, Stats> = HashMap::new();
    let mut lines_processed = 0;
    for chunk_stats_map in chunks_stat_maps {
        for (station, stats) in chunk_stats_map {
            let current_stats = final_stats_map.entry(station).or_insert(Stats::new());
            current_stats.min = current_stats.min.min(stats.min);
            current_stats.max = current_stats.max.max(stats.max);
            current_stats.sum += stats.sum;
            current_stats.count += stats.count;
            lines_processed += stats.count;
        }
    }

    {
        let output_file = File::create(output_filename).expect("cannot create output file");
        let mut writer = BufWriter::new(output_file);

        for (station, stats) in final_stats_map.iter() {
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
    }

    Ok(lines_processed)
}
