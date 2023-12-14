fn main() {
    let input = include_str!("./d13_input");
    let output = process(input);
    dbg!(output);
}

fn print_pattern(pattern: &Vec<Vec<char>>) {
    let dimensions = (pattern.len(), pattern[0].len());
    dbg!(&dimensions);

    for row in 0..dimensions.0 {
        for col in 0..dimensions.1 {
            print!("{}", pattern[row][col]);
        }
        print!("\n");
    }
}

fn process(input: &str) -> String {
    let mut pattern: Vec<Vec<char>> = vec![];

    let last_index = input.lines().count() - 1;
    let all_summary = input.lines().enumerate().fold(0, |acc, (index, line)| {
        let mut summary = 0;

        if !line.is_empty() {
            pattern.push(line.chars().collect());
        }

        if line.is_empty() || index == last_index {
            if !pattern.is_empty() {
                summary = process_pattern(&pattern);
            }

            pattern.clear();
        }

        acc + summary
    });

    dbg!(&all_summary);
    all_summary.to_string()
}

fn process_pattern(pattern: &Vec<Vec<char>>) -> usize {
    print_pattern(&pattern);
    println!("pattern");

    // Find 2 equal neighbouring rows
    let reflect_row = pattern
        .windows(2)
        .enumerate()
        .filter(|(_, w)| {
            w[0] == w[1] || w[0].iter().zip(w[1].iter()).filter(|(a, b)| a != b).count() <= 1
        })
        .find_map(|(index, _)| {
            // Split on reflection index, invert the other, trim to same length, compare
            let mut pattern_half1 = pattern.clone();
            let mut pattern_half2 = pattern_half1.split_off(index + 1);

            pattern_half1.reverse();
            pattern_half1.truncate(pattern_half2.len());
            pattern_half2.truncate(pattern_half1.len());

            let diff = pattern_half1
                .iter()
                .flatten()
                .zip(pattern_half2.iter().flatten())
                .filter(|(a, b)| a != b)
                .count();

            if diff == 1 {
                Some(index + 1)
            } else {
                None
            }
        });

    match reflect_row {
        Some(row) => {
            println!("pattern halves reflect rows at {}", row);
            return 100 * row;
        }
        None => (),
    }

    // Transpose and find 2 equal neighbouring cols
    let pattern_transposed: Vec<Vec<char>> = (0..pattern[0].len())
        .map(|i| {
            pattern
                .iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<char>>()
        })
        .collect();

    print_pattern(&pattern_transposed);
    println!("pattern transposed");

    let reflect_col = pattern_transposed
        .windows(2)
        .enumerate()
        .filter(|(_, w)| {
            w[0] == w[1] || w[0].iter().zip(w[1].iter()).filter(|(a, b)| a != b).count() <= 1
        })
        .find_map(|(index, _)| {
            // Split on reflection index, invert the other, trim to same length, compare
            let mut pattern_half1 = pattern_transposed.clone();
            let mut pattern_half2 = pattern_half1.split_off(index + 1);

            pattern_half1.reverse();
            pattern_half1.truncate(pattern_half2.len());
            pattern_half2.truncate(pattern_half1.len());

            let diff = pattern_half1
                .iter()
                .flatten()
                .zip(pattern_half2.iter().flatten())
                .filter(|(a, b)| a != b)
                .count();

            if diff == 1 {
                Some(index + 1)
            } else {
                None
            }
        });

    match reflect_col {
        Some(col) => {
            println!("pattern halves reflect cols at {}", col);
            return col;
        }
        None => (),
    }

    panic!("no reflections found");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        300
    )]
    #[case(
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        100
    )]
    fn test_pattern(#[case] input: &str, #[case] expected: usize) {
        let pattern = input.lines().map(|line| line.chars().collect()).collect();
        assert_eq!(expected, process_pattern(&pattern));
    }

    #[test]
    fn test_process() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!("400", process(input));
    }
}
