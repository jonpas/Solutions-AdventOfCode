use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./d12_input");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let total_arrangements = input.lines().fold(0, |acc, line| {
        let mut line_split = line.split(" ");
        let springs: Vec<char> = line_split
            .next()
            .expect("should be a valid string of characters")
            .chars()
            .collect();
        let groups: Vec<usize> = line_split
            .next()
            .expect("should be a valid string of comma-separated digits")
            .split(",")
            .map(|digit| digit.parse::<usize>().expect("should be a valid usize"))
            .collect();
        //dbg!(&springs, &groups);

        let unknowns: HashMap<usize, char> = springs
            .iter()
            .enumerate()
            .filter(|(_, spring)| *spring == &'?')
            .map(|(index, spring)| (index, *spring))
            .collect();
        //dbg!(&unknowns);

        let perms: Vec<String> = itertools::repeat_n([".", "#"], unknowns.len())
            .multi_cartesian_product()
            .map(|mcp| mcp.join(""))
            .collect();
        //dbg!(&perms);

        let arrangements = perms
            .iter()
            .filter(|perm| {
                //dbg!(&perm);
                let mut perm_iter = perm.chars();
                let filled_springs: String = springs
                    .iter()
                    .map(|spring| match spring {
                        '?' => perm_iter.next().expect("valid replacement should exist"),
                        _ => *spring,
                    })
                    .collect();

                let filled_spring_groups: Vec<&str> = filled_springs
                    .split(".")
                    .filter(|spring| !spring.is_empty())
                    .collect();
                //dbg!(&filled_spring_groups);

                let filled_groups: Vec<usize> = filled_spring_groups
                    .iter()
                    .map(|group| group.len())
                    .collect();
                //dbg!(&filled_groups);

                filled_groups == groups
            })
            .count();

        //dbg!(&arrangements);
        acc + arrangements
    });

    dbg!(&total_arrangements);
    total_arrangements.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("???.### 1,1,3", "1")]
    #[case(".??..??...?##. 1,1,3", "4")]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", "1")]
    #[case("????.#...#... 4,1,1", "1")]
    #[case("????.######..#####. 1,6,5", "4")]
    #[case("?###???????? 3,2,1", "10")]
    fn test_row(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process(input));
    }

    #[test]
    fn test_process() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("21", process(input));
    }
}
