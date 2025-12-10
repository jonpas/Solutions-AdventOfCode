fn main() {
    let input = include_str!("./d01_input");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut dial = 50;
    let output = input
        .lines()
        .map(|line| {
            let dir = &line[..1];
            let value = line[1..].parse::<i32>().unwrap();
            let rot = match dir {
                "R" => value,
                "L" => -value,
                &_ => 0,
            };

            dial += rot;
            dbg!(rot, dial);
            let mut wrap = 0;

            if dial < 0 {
                wrap = dial.abs() / 100 + 1;
                if dial % 100 == 0 {
                    wrap -= 1;
                }
                dial += 100 * wrap;
            } else if dial > 99 {
                wrap = dial / 100;
                dial -= 100 * wrap;
            }
            dbg!(dial, wrap);

            wrap
        })
        .sum::<i32>();
    output.to_string() // should be 6738
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("6", process(input));
    }

    #[test]
    fn test_1000() {
        let input = "R1000";
        assert_eq!("10", process(input));
    }
}
