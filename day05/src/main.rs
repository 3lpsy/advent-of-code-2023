use std::fs;
use std::path::Path;

use anyhow::Result;

#[derive(Debug, Copy, Clone)]
struct MapItem {
    // first
    destination_range_start: u64,
    // second number
    source_range_start: u64,
    // says how big the source and destination ranges are
    range_length: u64,
}

impl MapItem {
    pub fn parse(line: String) -> Self {
        let digits = line
            .trim()
            .split(' ')
            .map(|c| c.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let destination_range_start = digits[0];
        let source_range_start = digits[1];
        let range_length = digits[2];
        Self {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }
    pub fn resolve_source(&self, source_value: u64) -> (bool, u64) {
        // find in source_range, return offset from destination
        //
        if source_value < self.source_range_start
            || source_value > (self.source_range_start + self.range_length)
        {
            //outside of range
            return (false, source_value);
        }
        let offset = source_value - self.source_range_start;
        (true, self.destination_range_start + offset as u64)
    }
}
fn main() -> Result<()> {
    // input lists all seeds that need to be planted
    // soil to use seed
    // fertilizer to use with soil=
    // type of water to use with each fertizler
    // numbers reused between categories (not unique between)
    //
    // seed to soil map describes how to convert a seed number (source) to a soil number
    // (destination)
    //
    let mut input = fs::read_to_string(Path::new("day05/input.txt"))?;
    let mut almanac_iter = input.lines().peekable();
    //part 1
    // let data = almanac_iter
    //     .next()
    //     .unwrap()
    //     .split_once(' ')
    //     .unwrap()
    //     .1
    //     .split(' ')
    //     .map(|s| s.trim())
    //     .map(|s| s.parse::<u64>().unwrap())
    //     .collect::<Vec<u64>>();
    //
    let data_sets = almanac_iter
        .next()
        .unwrap()
        .split_once(' ')
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.trim())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // let mut data = Vec::new();

    let mut maps: Vec<Vec<MapItem>> = Vec::new();
    let mut rows: Option<Vec<MapItem>> = None;

    for line in almanac_iter.clone() {
        if line.is_empty() {
            if rows.is_some() {
                maps.push(rows.unwrap());
            }
            rows = None;
        } else if line.ends_with("map:") {
            rows = Some(Vec::new());
        } else {
            rows.as_mut()
                .unwrap()
                .push(MapItem::parse(line.to_string()));
        }
    }
    // add last one
    maps.push(rows.clone().unwrap());

    let layers = maps.len();
    let mut locations: Vec<u64> = Vec::new();
    // part 2 requirement
    for item_set in data_sets.chunks(2) {
        let start = item_set[0];
        let offset = item_set[1];
        let end = start + offset;

        println!("Starting: {start} to {end}");
        for item in start..end {
            let mut current = item;
            for map in maps.iter() {
                // ecah map is a level that holds lines
                for row in map.iter() {
                    let (status, resolved) = row.resolve_source(current);
                    current = resolved; // set currrent to resolved, it'll be the same or the new value
                    if status {
                        // if it was resolved, we've found the next one, don't continue going on lines
                        break;
                    }
                }
                // we've resolved this level and set it to current
            }
            locations.push(current);
        }
    }

    let min = locations.iter().min().unwrap();
    dbg!(min);
    Ok(())
}
