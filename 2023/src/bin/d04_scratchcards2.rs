fn main() {
    let input = include_str!("./d04_input");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut next = vec![1; input.lines().count()];

    input.lines().enumerate().for_each(|(index, line)| {
        let have_winning = process_line(line);

        for _ in 0..next[index] {
            for i in index + 1..index + (have_winning as usize) + 1 {
                next[i] += 1;
            }
        }
        dbg!(&next);
    });

    next.iter().sum::<u32>().to_string()
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

    dbg!(have_winning);
    have_winning as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input));
    }
}
