fn main() {
    let input = include_str!("./d02_input").trim();
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let output = input
        .split(",")
        .map(|range| {
            dbg!(range);
            let mut range_parts = range.split("-");
            let (start, end) = (
                range_parts.next().unwrap().parse::<u64>().unwrap(),
                range_parts.next().unwrap().parse::<u64>().unwrap(),
            );
            let mut invalids = 0;
            for id in start..=end {
                let ids = id.to_string();
                let len = ids.len() / 2;
                let (id1, id2) = (&ids[..len], &ids[len..]);
                if id1 == id2 {
                    invalids += id;
                }
            }

            invalids
        })
        .sum::<u64>();
    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input));
    }
}
