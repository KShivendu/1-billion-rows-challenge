use rayon::{current_thread_index, prelude::*}; // current_thread_index,
use std::io::{BufWriter, Read, Seek, Write};
use std::str::Lines;
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

pub fn generate_chunks(file: File) -> Result<Vec<(usize, usize)>, io::Error> {
    let cpu_count: usize = 8; // Decides the number of chunks
    let file_size = file.metadata()?.len() as usize;
    let chunk_size = file_size / cpu_count;

    let mut chunks: Vec<(usize, usize)> = vec![];
    let mut start = 0;

    let mut reader = BufReader::new(file);

    dbg!("started generating chunks");

    while start < file_size {
        // Read bytes from the current start position till buffer size at max:
        let mut buffer = vec![0; chunk_size];
        reader.seek(io::SeekFrom::Start(start as u64))?;
        let bytes_read = reader.read(&mut buffer[..])?; // bytes_read <= chunk_size

        // Find the position of the last newline in the buffer
        let mut last_newline_in_buffer = bytes_read; // If no newline is found, use the entire buffer (FIXME: This can be troublesome if a huge line is inserted that exceeds chunk size?)
        for i in (0..bytes_read).rev() {
            if buffer[i] == b'\n' {
                last_newline_in_buffer = i + 1;
                break;
            }
        }

        // Calculate end position:
        let end = start + last_newline_in_buffer;
        chunks.push((start, end));

        start = end;
    }

    dbg!("done generating chunks"); // Looks like this takes a long time

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
    let input_file = File::open(input_filename).unwrap_or_else(|_| {
        let pwd = std::env::current_dir().expect("cannot get current directory");
        panic!(
            "cannot open input file {}. pwd: {}",
            input_filename,
            pwd.to_str().unwrap()
        )
    });

    let chunks = generate_chunks(input_file).expect("cannot generate chunks");

    dbg!(&chunks);

    let output_file = File::create(output_filename).expect("cannot create output file");
    let mut writer = BufWriter::new(output_file);

    let chunks_stat_maps: Vec<HashMap<String, Stats>> = chunks
        .par_iter()
        .map(|(start, end)| {
            let file = File::open(input_filename).unwrap();
            let mut reader = BufReader::new(file);

            reader.seek(io::SeekFrom::Start(*start as u64)).unwrap();
            let mut buffer = Vec::new();
            reader
                .take((*end - *start) as u64)
                .read_to_end(&mut buffer)
                .unwrap();
            let content = String::from_utf8_lossy(&buffer);
            // let thread_id = current_thread_index();
            // dbg!(thread_id, &content.len());
            let chunk_stats_map = process_chunk(content.lines(), content.len());
            chunk_stats_map
        })
        .collect();

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

    Ok(lines_processed)
}
