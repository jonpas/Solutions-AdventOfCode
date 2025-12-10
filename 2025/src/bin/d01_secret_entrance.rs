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

            dbg!(rot);
            dial += rot;

            if dial > 99 {
                dial -= (dial / 100) * 100;
            } else if dial < 0 {
                dial += (-dial / 100) * 100;
            }
            dbg!(dial);

            if dial == 0 {
                1
            } else {
                0
            }
        })
        .sum::<u32>();
    output.to_string()
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
        assert_eq!("3", process(input));
    }
}
