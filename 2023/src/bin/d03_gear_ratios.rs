use regex::Regex;

fn main() {
    let input = include_str!("./d03_input");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let re = Regex::new(r"\d+").unwrap();

    let output = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            re.find_iter(line)
                .map(|m| {
                    let mut valid = false;
                    dbg!(m.as_str());

                    // Check for symbol before or after on same line
                    if m.start() > 0 {
                        let prev_symbol =
                            line.chars().nth(m.start() - 1).expect("should be a char");
                        if prev_symbol != '.' {
                            valid = true;
                        }
                    };
                    if m.end() < line.len() {
                        let next_symbol = line.chars().nth(m.end()).expect("should be a char");
                        if next_symbol != '.' {
                            valid = true;
                        }
                    }

                    // Check for symbol in line above or below
                    if index > 0 {
                        let prev_line = input.lines().nth(index - 1).expect("should be a string");
                        //dbg!(prev_line);

                        let start = m.start().max(1) - 1;
                        let end = m.end().min(prev_line.len() - 1) + 1;

                        for pos in start..end {
                            let symbol = prev_line.chars().nth(pos).expect("should be a char");
                            //dbg!(symbol);
                            if symbol != '.' && !symbol.is_digit(10) {
                                valid = true;
                            }
                        }
                    }

                    if index < input.lines().count() - 1 {
                        let next_line = input.lines().nth(index + 1).expect("should be a string");
                        //dbg!(next_line);

                        let start = m.start().max(1) - 1;
                        let end = m.end().min(next_line.len() - 1) + 1;

                        for pos in start..end {
                            let symbol = next_line.chars().nth(pos).expect("should be a char");
                            //dbg!(symbol);
                            if symbol != '.' && !symbol.is_digit(10) {
                                valid = true;
                            }
                        }
                    }

                    if valid {
                        dbg!(valid);
                        m.as_str().parse::<u32>().expect("should be a number")
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input));
    }
}
