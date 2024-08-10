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
