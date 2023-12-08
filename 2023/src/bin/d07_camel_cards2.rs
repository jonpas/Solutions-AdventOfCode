use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./d07_input");
    let output = process(input);
    dbg!(output);
}

// Default ordered by discriminant (top = smallest, bottom = largest)
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Card {
    J, // part 2 makes this the weakest card
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

// Default ordered by discriminant (top = smallest, bottom = largest)
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then(self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn new(cards_string: String, bid: u32) -> Hand {
        // Map card characters to enum values for easier comparison
        let cards: Vec<Card> = cards_string
            .chars()
            .map(|card| match card {
                '2' => Card::N2,
                '3' => Card::N3,
                '4' => Card::N4,
                '5' => Card::N5,
                '6' => Card::N6,
                '7' => Card::N7,
                '8' => Card::N8,
                '9' => Card::N9,
                'T' => Card::T,
                'J' => Card::J,
                'Q' => Card::Q,
                'K' => Card::K,
                'A' => Card::A,
                _ => unreachable!(),
            })
            .collect();

        // Preprocess hand type
        let mut set: HashMap<char, u8> = HashMap::new();
        let mut jokers = 0;
        cards_string.chars().for_each(|card| {
            if card == 'J' {
                jokers += 1;
            } else {
                set.entry(card).and_modify(|freq| *freq += 1).or_insert(1);
            }
        });

        if jokers == 5 {
            set.entry('J').or_insert(5);
        } else {
            match set.iter().max_by_key(|s| s.1) {
                Some(most_freq_card) => {
                    set.entry(*most_freq_card.0)
                        .and_modify(|freq| *freq += jokers);
                }
                _ => (),
            }
        }

        let mut hand_type = HandType::HighCard;
        if set.len() == 1 {
            hand_type = HandType::FiveOfAKind;
        } else if set.len() == 2 {
            // Four of a kind or Full house
            if set.values().any(|&freq| freq == 4) {
                hand_type = HandType::FourOfAKind;
            } else {
                hand_type = HandType::FullHouse;
            }
        } else if set.len() == 3 {
            // Three of a kind or Two pair
            if set.values().any(|&freq| freq == 3) {
                hand_type = HandType::ThreeOfAKind;
            } else {
                hand_type = HandType::TwoPair;
            }
        } else if set.len() == 4 {
            hand_type = HandType::OnePair;
        }

        dbg!(&cards, &set, &hand_type);

        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}

fn process(input: &str) -> String {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let mut line_split = line.split(" ");
            let cards_string = line_split.next().expect("characters").to_string();
            let bid = line_split
                .next()
                .expect("number")
                .parse::<u32>()
                .ok()
                .expect("number");

            Hand::new(cards_string, bid)
        })
        .collect();

    hands.sort();
    dbg!(&hands);

    let total_win = hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        let hand_win = hand.bid as usize * (index + 1);
        acc + hand_win
    });

    total_win.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("32T3K", HandType::OnePair)]
    #[case("T55J5", HandType::FourOfAKind)]
    #[case("KK677", HandType::TwoPair)]
    #[case("KTJJT", HandType::FourOfAKind)]
    #[case("QQQJA", HandType::FourOfAKind)]
    // custom
    #[case("JJJJA", HandType::FiveOfAKind)]
    #[case("JJJJJ", HandType::FiveOfAKind)]
    fn test_joker(#[case] input: String, #[case] expected: HandType) {
        assert_eq!(expected, Hand::new(input, 0).hand_type);
    }

    #[test]
    fn test_process() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("5905", process(input));
    }
}
