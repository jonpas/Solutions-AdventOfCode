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

            let mut bats = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            for (i, &digit) in digits.iter().enumerate() {
                if digit > bats[0] && digits.len() - 1 - i >= 11 {
                    bats[0] = digit;
                    for j in 1..=11 {
                        bats[j] = 0;
                    }
                } else if digit > bats[1] && digits.len() - 1 - i >= 10 {
                    bats[1] = digit;
                    for j in 2..=11 {
                        bats[j] = 0;
                    }
                } else if digit > bats[2] && digits.len() - 1 - i >= 9 {
                    bats[2] = digit;
                    for j in 3..=11 {
                        bats[j] = 0;
                    }
                } else if digit > bats[3] && digits.len() - 1 - i >= 8 {
                    bats[3] = digit;
                    for j in 4..=11 {
                        bats[j] = 0;
                    }
                } else if digit > bats[4] && digits.len() - 1 - i >= 7 {
                    bats[4] = digit;
                    for j in 5..=11 {
                        bats[j] = 0;
                    }
                } else if digit > bats[5] && digits.len() - 1 - i >= 6 {
                    bats[5] = digit;
                    for j in 6..=11 {
                        bats[j] = 0;
                    }
                } else if digit > bats[6] && digits.len() - 1 - i >= 5 {
                    bats[6] = digit;
                    for j in 7..=11 {
                        bats[j] = 0;
                    }
                } else if digit > bats[7] && digits.len() - 1 - i >= 4 {
                    bats[7] = digit;
                    for j in 8..=11 {
                        bats[j] = 0;
                    }
                } else if digit > bats[8] && digits.len() - 1 - i >= 3 {
                    bats[8] = digit;
                    for j in 9..=11 {
                        bats[j] = 0;
                    }
                } else if digit > bats[9] && digits.len() - 1 - i >= 2 {
                    bats[9] = digit;
                    for j in 10..=11 {
                        bats[j] = 0;
                    }
                } else if digit > bats[10] && digits.len() - 1 - i >= 1 {
                    bats[10] = digit;
                    bats[11] = 0;
                } else if digit > bats[11] {
                    bats[11] = digit;
                }
            }

            let joltage = bats
                .iter()
                .map(|bat| bat.to_string())
                .collect::<Vec<_>>()
                .join("")
                .parse::<u64>()
                .unwrap();
            dbg!(joltage);
            joltage
        })
        .sum::<u64>();
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
        assert_eq!("3121910778619", process(input));
    }
}
