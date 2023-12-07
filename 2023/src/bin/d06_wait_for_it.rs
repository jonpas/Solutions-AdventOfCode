fn main() {
    let input = include_str!("./d06_input");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut lines = input.lines();
    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .strip_prefix("Time: ")
        .expect("parse times")
        .split(' ')
        .filter_map(|time| time.trim().parse::<u32>().ok())
        .collect();
    let records: Vec<u32> = lines
        .next()
        .unwrap()
        .strip_prefix("Distance: ")
        .expect("parse record distances")
        .split(' ')
        .filter_map(|record| record.trim().parse::<u32>().ok())
        .collect();

    dbg!(&times, &records);

    let product: u32 = times
        .iter()
        .zip(records)
        .map(|(time, record)| {
            dbg!(time, record);
            let mut ways_to_beat = 0;
            let mut past_win_ways = false;
            for held in 0..*time {
                let distance = held * (time - held);
                if distance > record {
                    past_win_ways = true;
                    ways_to_beat += 1;
                } else if past_win_ways {
                    break;
                }
                dbg!(held, distance);
            }
            ways_to_beat
        })
        .product();

    product.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", process(input));
    }
}
