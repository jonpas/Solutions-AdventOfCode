fn main() {
    let input = include_str!("./d15_input");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let hash_sum = input
        .split(",")
        .map(|step| hash(step.trim_end()))
        .sum::<usize>();
    hash_sum.to_string()
}

fn hash(seq: &str) -> usize {
    seq.chars()
        .fold(0, |acc, s| ((acc + s as usize) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("HASH", "52")]
    #[case("rn=1", "30")]
    #[case("cm-", "253")]
    #[case("qp=3", "97")]
    #[case("cm=2", "47")]
    #[case("qp-", "14")]
    #[case("pc=4", "180")]
    #[case("ot=9", "9")]
    #[case("ab=5", "197")]
    #[case("pc-", "48")]
    #[case("pc=6", "214")]
    #[case("ot=7", "231")]
    fn test_hash(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, hash(input).to_string());
    }

    #[test]
    fn test_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("1320", process(input));
    }
}
