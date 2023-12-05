use std::ops::Range;

fn main() {
    let input = include_str!("./d05_input");
    let output = process(input);
    dbg!(output);
}

struct MapRange {
    source: Range<u64>,
    dest: Range<u64>,
}

fn process(input: &str) -> String {
    let mut seeds: Vec<u64> = vec![];
    let mut map: Vec<MapRange> = vec![];

    input.lines().for_each(|line| {
        //dbg!(line);
        if line.starts_with("seeds:") {
            seeds = line.split(':').collect::<Vec<&str>>()[1]
                .split(' ')
                .filter_map(|number| number.parse::<u64>().ok())
                .collect();
        } else if line.contains("map:") {
            map.clear();
        } else if line.is_empty() {
            if !map.is_empty() {
                seeds = process_map(&seeds, &map);
            }
        } else {
            let range: Vec<u64> = line
                .split(' ')
                .filter_map(|number| number.parse::<u64>().ok())
                .collect();
            //dbg!(&range);

            let source_start = range[1];
            let dest_start = range[0];
            let length = range[2];

            map.push(MapRange {
                source: source_start..source_start + length,
                dest: dest_start..dest_start + length,
            });
        }
    });

    // Process last map (no empty line after)
    seeds = process_map(&seeds, &map);
    dbg!(&seeds);

    seeds.iter().min().expect("should be a number").to_string()
}

fn process_map(seeds: &Vec<u64>, map: &Vec<MapRange>) -> Vec<u64> {
    dbg!(&seeds);
    //dbg!(&map);
    seeds
        .iter()
        .map(|seed| {
            let mut mapped_seed = seed.clone();
            map.iter().for_each(|m| {
                if m.source.contains(&seed) {
                    //dbg!(format!("contains {seed}"));
                    //dbg!(m.dest.start, m.source.start, seed);
                    mapped_seed = m.dest.start + (seed - m.source.start);
                }
            });
            dbg!(seed, mapped_seed);
            mapped_seed
        })
        .collect()
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
        assert_eq!("35", process(input));
    }
}
