fn main() {
    let input = include_str!("./d04_input");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let output = input.lines().map(|line| process_line(line)).sum::<u32>();
    output.to_string()
}

fn process_line(line: &str) -> u32 {
    let numbers: Vec<&str> = line.split('|').collect();

    let winning: Vec<u32> = numbers[0].split(':').collect::<Vec<&str>>()[1]
        .split(' ')
        .filter_map(|number| number.parse::<u32>().ok())
        .collect();

    let have: Vec<u32> = numbers[1]
        .split(' ')
        .filter_map(|number| number.parse::<u32>().ok())
        .collect();

    let have_winning = have.iter().filter(|h| winning.contains(h)).count();

    let mut result = if have_winning > 0 { 1 } else { 0 };
    for _ in 1..have_winning {
        result *= 2;
    }

    dbg!(winning, have, have_winning, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn test_process_line(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, process_line(input));
    }

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input));
    }
}
