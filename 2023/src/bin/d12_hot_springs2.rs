use std::collections::HashMap;

type Cache = HashMap<(Vec<char>, Vec<usize>), usize>;

fn main() {
    let input = include_str!("./d12_input");
    let output = process(input);
    dbg!(output);
}

fn unfold(mut input: Vec<char>) -> Vec<char> {
    input.push('?');
    input = input.repeat(5);
    input.pop();
    //dbg!(&input);
    input
}

fn process(input: &str) -> String {
    let mut cache = HashMap::new();

    let total_arrangements = input.lines().fold(0, |acc, line| {
        let mut line_split = line.split(" ");
        let springs: Vec<char> = unfold(
            line_split
                .next()
                .expect("should be a valid string of characters")
                .chars()
                .collect(),
        );
        let groups: Vec<usize> = line_split
            .next()
            .expect("should be a valid string of comma-separated digits")
            .split(",")
            .map(|digit| digit.parse::<usize>().expect("should be a valid usize"))
            .collect::<Vec<usize>>()
            .repeat(5);
        //dbg!(&springs, &groups);

        let arrangements = arrange(&springs, &groups, &mut cache);
        dbg!(arrangements);
        acc + arrangements
    });

    dbg!(&total_arrangements);
    total_arrangements.to_string()
}

fn arrange(springs: &Vec<char>, groups: &Vec<usize>, cache: &mut Cache) -> usize {
    if let Some(arrangement) = cache.get(&(springs.clone(), groups.clone())) {
        return *arrangement;
    }

    if groups.is_empty() {
        return (!springs.contains(&'#')) as usize;
    }

    let min_remaining = groups.iter().sum::<usize>() + groups.len() - 1;

    if springs.len() < min_remaining {
        return 0;
    }

    let arrangement = match springs[0] {
        '.' => arrange(&springs[1..].to_vec(), groups, cache),
        '#' => hash(springs, groups, cache),
        '?' => arrange(&springs[1..].to_vec(), groups, cache) + hash(springs, groups, cache),
        _ => unreachable!("invalid character"),
    };

    cache.insert((springs.to_vec(), groups.to_vec()), arrangement);
    arrangement
}

fn hash(springs: &Vec<char>, groups: &Vec<usize>, cache: &mut Cache) -> usize {
    if springs.len() < groups[0] || springs[0..groups[0]].contains(&'.') {
        return 0;
    }

    if springs.len() == groups[0] {
        return (groups.len() == 1) as usize;
    }

    if springs[groups[0]] == '#' {
        return 0;
    }

    arrange(
        &springs[groups[0] + 1..].to_vec(),
        &groups[1..].to_vec(),
        cache,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(".#", ".#?.#?.#?.#?.#")]
    #[case("???.###", "???.###????.###????.###????.###????.###")]
    fn test_unfold(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(
            expected.chars().collect::<Vec<char>>(),
            unfold(input.chars().collect::<Vec<char>>())
        );
    }

    #[rstest]
    #[case("???.### 1,1,3", "1")]
    #[case(".??..??...?##. 1,1,3", "16384")]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", "1")]
    #[case("????.#...#... 4,1,1", "16")]
    #[case("????.######..#####. 1,6,5", "2500")]
    #[case("?###???????? 3,2,1", "506250")]
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
        assert_eq!("525152", process(input));
    }
}
