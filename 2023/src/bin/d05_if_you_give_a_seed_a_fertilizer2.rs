use std::ops::Range;

fn main() {
    let input = include_str!("./d05_input");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct MapRange {
    source: Range<u64>,
    dest: Range<u64>,
}

fn process(input: &str) -> String {
    let mut seeds_ranges: Vec<Range<u64>> = vec![];
    let mut map: Vec<Vec<MapRange>> = vec![];

    input.lines().for_each(|line| {
        //dbg!(line);
        if line.starts_with("seeds:") {
            let seeds: Vec<u64> = line.split(':').collect::<Vec<&str>>()[1]
                .split(' ')
                .filter_map(|number| number.parse::<u64>().ok())
                .collect();

            for index in (0..seeds.len()).step_by(2) {
                let start = seeds[index];
                let end = seeds[index] + seeds[index + 1];
                seeds_ranges.push(start..end);
            }
        } else if line.contains("map:") {
            map.push(vec![]);
        } else if !line.is_empty() {
            let range: Vec<u64> = line
                .split(' ')
                .filter_map(|number| number.parse::<u64>().ok())
                .collect();
            //dbg!(&range);

            let source_start = range[1];
            let dest_start = range[0];
            let length = range[2];

            map.last_mut().unwrap().push(MapRange {
                source: source_start..source_start + length,
                dest: dest_start..dest_start + length,
            });
        }
    });

    dbg!(&map);
    let mut lowest_seed = u64::MAX;

    seeds_ranges.iter().for_each(|seeds_range| {
        for mut seed in seeds_range.clone() {
            //dbg!(seed);

            map.iter().for_each(|submap| {
                seed = process_map(seed, &submap);
            });

            lowest_seed = lowest_seed.min(seed);
        }
    });

    dbg!(lowest_seed);
    lowest_seed.to_string()
}

fn process_map(seed: u64, map: &Vec<MapRange>) -> u64 {
    let mut mapped_seed = seed.clone();
    map.iter().for_each(|m| {
        if m.source.contains(&seed) {
            mapped_seed = m.dest.start + (seed - m.source.start);
        }
    });
    mapped_seed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("46", process(input));
    }
}
