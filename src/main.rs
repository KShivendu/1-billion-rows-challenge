use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("file not found");
    let reader = BufReader::new(file);

    // u32::MAX is 4.29B so it should be enough for 1BRC

    let mut sum_and_count_map: HashMap<String, (f32, u32)> = HashMap::new();
    let mut max_map: HashMap<String, f32> = HashMap::new();
    let mut min_map: HashMap<String, f32> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("cannot read line");
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

        // println!("station: {}, number: {}", station, number);
    }

    for (station, (sum, count)) in sum_and_count_map.into_iter() {
        let min = min_map.get(&station).unwrap();
        let max = max_map.get(&station).unwrap();
        println!("{};{};{};{}", station, min, (sum / (count as f32)), max);
    }

    Ok(())
}
