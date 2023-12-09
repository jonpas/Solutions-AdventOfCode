fn main() {
    let input = include_str!("./d09_input");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let histories: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|n| n.parse::<i32>().ok().expect("should be a number"))
                .collect()
        })
        .collect();
    //dbg!(&histories);

    let sum_predictions = histories
        .iter()
        .map(|history| {
            let mut firsts: Vec<i32> = vec![];

            let mut diff: Vec<i32> = history.clone();
            while !diff.iter().all(|x| *x == 0) {
                firsts.push(*diff.first().expect("should be a number"));
                diff = diff.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
                //dbg!(&diff);
            }
            //dbg!(&lasts);

            firsts.iter().rev().fold(0, |acc, first| first - acc)
        })
        .sum::<i32>();

    dbg!(&sum_predictions);
    sum_predictions.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("2", process(input));
    }
}
