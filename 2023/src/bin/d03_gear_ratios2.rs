use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./d03_input");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Gear {
    adjacent: u32,
    value: u32,
}

fn process(input: &str) -> String {
    let re = Regex::new(r"\d+").unwrap();

    let mut map: HashMap<(usize, usize), Gear> = HashMap::new();

    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            re.find_iter(line)
                .map(|m| {
                    dbg!(m.as_str());
                    let num = m.as_str().parse::<u32>().expect("should be a number");

                    // Check for symbol before or after on same line
                    if m.start() > 0 {
                        let prev_symbol =
                            line.chars().nth(m.start() - 1).expect("should be a char");
                        if prev_symbol == '*' {
                            let gear = map.entry((index, m.start() - 1)).or_insert(Gear {
                                adjacent: 0,
                                value: 1,
                            });
                            (*gear).value *= num;
                            (*gear).adjacent += 1;
                        }
                    };
                    if m.end() < line.len() {
                        let next_symbol = line.chars().nth(m.end()).expect("should be a char");
                        if next_symbol == '*' {
                            let gear = map.entry((index, m.end())).or_insert(Gear {
                                adjacent: 0,
                                value: 1,
                            });
                            (*gear).value *= num;
                            (*gear).adjacent += 1;
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
                            if symbol == '*' {
                                let gear = map.entry((index - 1, pos)).or_insert(Gear {
                                    adjacent: 0,
                                    value: 1,
                                });
                                (*gear).value *= num;
                                (*gear).adjacent += 1;
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
                            if symbol == '*' {
                                let gear = map.entry((index + 1, pos)).or_insert(Gear {
                                    adjacent: 0,
                                    value: 1,
                                });
                                (*gear).value *= num;
                                (*gear).adjacent += 1;
                            }
                        }
                    }

                    0
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    //dbg!(&map);

    let total = map
        .values()
        .map(|gear| if gear.adjacent == 2 { gear.value } else { 0 })
        .sum::<u32>();

    total.to_string()
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
        assert_eq!("467835", process(input));
    }
}
