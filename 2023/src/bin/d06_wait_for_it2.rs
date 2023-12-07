fn main() {
    let input = include_str!("./d06_input");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .strip_prefix("Time: ")
        .expect("parse times")
        .replace(" ", "")
        .parse::<u64>()
        .expect("should be a number");
    let record = lines
        .next()
        .unwrap()
        .strip_prefix("Distance: ")
        .expect("parse record distances")
        .replace(" ", "")
        .parse::<u64>()
        .expect("should be a number");

    dbg!(&time, &record);

    let mut ways_to_beat = 0;
    let mut past_win_ways = false;
    for held in 0..time {
        let distance = held * (time - held);
        if distance > record {
            past_win_ways = true;
            ways_to_beat += 1;
        } else if past_win_ways {
            break;
        }
        //dbg!(held, distance);
    }

    ways_to_beat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", process(input));
    }
}
