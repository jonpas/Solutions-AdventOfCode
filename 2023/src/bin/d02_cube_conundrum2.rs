use regex::Regex;

fn main() {
    let input = include_str!("./d02_input");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let re_cubes = Regex::new(r"\d+ (red|green|blue)").unwrap();

    let output = input
        .lines()
        .map(|line| {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            re_cubes.find_iter(line).for_each(|m| {
                let cube: Vec<&str> = m.as_str().split(' ').collect();
                let count = cube[0].parse::<u32>().expect("should be a number");
                let color = cube[1];

                if color == "red" {
                    max_red = max_red.max(count);
                } else if color == "green" {
                    max_green = max_green.max(count);
                } else if color == "blue" {
                    max_blue = max_blue.max(count);
                }
            });

            max_red * max_green * max_blue
        })
        .sum::<u32>();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input));
    }
}
