use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./d15_input");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Lens<'a> {
    label: &'a str,
    focal: usize,
}

fn process(input: &str) -> String {
    let mut boxes: BTreeMap<usize, Vec<Lens>> = BTreeMap::new();

    input.split(",").for_each(|step| {
        let pos = step
            .chars()
            .position(|c| ['=', '-'].contains(&c))
            .expect("should contain = or -");
        let mut step_chars = step.chars();

        let label = &step[..pos];
        let op = step_chars.nth(pos).expect("should be = or -");
        let focal = match op {
            '=' => step_chars
                .next()
                .expect("should be a digit")
                .to_digit(10)
                .expect("should be a number"),
            _ => 0,
        };

        let lens = Lens {
            label,
            focal: focal as usize,
        };

        let hash = hash(label.trim_end());
        dbg!((&label, &op, &focal, &hash, &lens));

        boxes
            .entry(hash)
            .and_modify(
                |lenses| match lenses.iter().position(|l| l.label == lens.label) {
                    Some(index) => match op {
                        '=' => {
                            lenses[index] = lens.clone();
                        }
                        _ => {
                            lenses.remove(index);
                        }
                    },
                    None => match op {
                        '=' => {
                            lenses.push(lens.clone());
                        }
                        _ => {}
                    },
                },
            )
            .or_insert(vec![lens]);
    });

    //dbg!(&boxes);

    let focus_power = boxes
        .iter()
        .map(|(index, bx)| {
            bx.iter()
                .enumerate()
                .map(|(slot, lens)| (index + 1) * (slot + 1) * lens.focal)
                .sum::<usize>()
        })
        .sum::<usize>();

    focus_power.to_string()
}

// 261821 too high

fn hash(seq: &str) -> usize {
    seq.chars()
        .fold(0, |acc, s| ((acc + s as usize) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("145", process(input));
    }
}
