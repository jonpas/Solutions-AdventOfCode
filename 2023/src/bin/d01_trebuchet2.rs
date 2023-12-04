fn main() {
    let input = include_str!("./d01_input");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let translated_input: String = input
        .lines()
        .map(|line| {
            let mut new_line = String::new();

            let mut it = line.chars();
            while !it.as_str().is_empty() {
                let it_str = it.as_str();

                let new_char = if it_str.starts_with("one") {
                    it.next();
                    '1'
                } else if it_str.starts_with("two") {
                    it.next();
                    '2'
                } else if it_str.starts_with("three") {
                    it.next();
                    '3'
                } else if it_str.starts_with("four") {
                    it.next();
                    '4'
                } else if it_str.starts_with("five") {
                    it.next();
                    '5'
                } else if it_str.starts_with("six") {
                    it.next();
                    '6'
                } else if it_str.starts_with("seven") {
                    it.next();
                    '7'
                } else if it_str.starts_with("eight") {
                    it.next();
                    '8'
                } else if it_str.starts_with("nine") {
                    it.next();
                    '9'
                } else {
                    it.nth(0).expect("should be a char")
                };

                new_line.push(new_char);
            }

            new_line.push('\n');
            new_line
        })
        .collect();

    let output = translated_input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|character| character.to_digit(10));
            let first = it.next().expect("should be a number");
            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("should be a number")
        })
        .sum::<u32>();
    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_mod() {
        assert_eq!("29", process("two1nine"));
        assert_eq!("83", process("eightwothree"));
        assert_eq!("13", process("abcone2threexyz"));
        assert_eq!("24", process("xtwone3four"));
        assert_eq!("42", process("4nineeightseven2"));
        assert_eq!("14", process("zoneight234"));
        assert_eq!("76", process("7pqrstsixteen"));
    }

    #[test]
    fn test_process() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input));
    }
}
