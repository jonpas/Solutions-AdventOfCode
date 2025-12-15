fn main() {
    let input = include_str!("./d03_input");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let output = input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            let mut first: u32 = 0;
            let mut second: u32 = 0;
            for (i, &digit) in digits.iter().enumerate() {
                if digit > first && i < digits.len() - 1 {
                    first = digit;
                    second = 0;
                } else if digit > second {
                    second = digit;
                }
            }

            let joltage = vec![first.to_string(), second.to_string()]
                .join("")
                .parse::<u32>()
                .unwrap();
            joltage
        })
        .sum::<u32>();
    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("357", process(input));
    }
}
