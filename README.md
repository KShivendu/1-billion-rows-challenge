## 1 Billion Row challenge in Rust 🦀

Read https://1brc.dev for more info

I'll improve this version over time


## Usage:

```bash
# Download the data
# Note that this has only 1 entry for each city. So min/max/mean are all the same. You can use it only for demo
mkdir -p data
wget https://raw.githubusercontent.com/gunnarmorling/1brc/main/data/weather_stations.csv -O data/weather_stations.csv

# Run the program
cargo run data/weather_stations.csv
```

### Perf analysis:

```bash
cargo install flamegraph
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
echo 0 | sudo tee /proc/sys/kernel/kptr_restrict

# perf stats:
perf stat ./target/perf/one_brc data/weather_stations.csv

# More:
perf record -g cargo run --profile perf data/weather_stations.csv
perf report -g
```

## Iterations

* Naive implementation perf:
    ```
    Processed 44691 lines in 835 ms
    Expected seconds for 1 billion rows instead 18696.61464276924 (5.2hours)
    ```

* Write to file instead of terminal:
    ```
    Processed 44691 lines in 197 ms
    Expected seconds for 1 billion rows instead 4408.8128706003445 (1.2hours)
    # Improvement: 835 / 197 = 4.2X
    ```

* Use release build instead of debug:
    ```
    Processed 44691 lines in 71 ms
    Expected seconds for 1 billion rows instead 1593.0595198138326 (27mins)
    # Improvement: 197 / 71 = 2.77X
    ```

* Use 1 hashmap (with struct) instead of 3
    ```bash
    # My machine's perf degradaded a bit so absolute perf went down. But relative improvement can be seen

    # Before:
    Processed 44691 lines in 119 ms
    Expected seconds for 1 billion rows instead 2676.9450448636194
    # After:
    Processed 44691 lines in 56 ms
    Expected seconds for 1 billion rows instead 1264.9479537267011
    # Improvement: 119 / 56 = 2.1X
    ```

* Use chunks with parallel rayon workers
  ```bash

    ➜  one_brc git:(master) ✗ ./target/release/one_brc measurements.txt                                                            1162ms
    Time taken in generating chunks: 108μs
    [src/processor.rs:126:5] &chunks = [
        (
            0,
            1724479094,
        ),
        (
            1724479095,
            3448958200,
        ),
    ]
    [src/processor.rs:143:13] current_thread_index().unwrap() = 0
    [src/processor.rs:143:13] buffer.len() = 1724479094
    Current physical memory usage: 3209MB
    Current virtual memory usage: 5155MB
    [src/processor.rs:143:13] current_thread_index().unwrap() = 2
    [src/processor.rs:143:13] buffer.len() = 1724479105
    Current physical memory usage: 3291MB
    Current virtual memory usage: 5155MB
    Processed 250M lines in 20451 ms
    Expected seconds for 1 billion rows instead 81.8054588989169

    # 1264/81=15X improvement!
  ```
