use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap},
    str::FromStr,
};

#[derive(Debug)]
enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPairs,
    OnePair,
    HighCard,
}

impl HandType {
    fn to_val(&self) -> i32 {
        match self {
            HandType::Five => 7,
            HandType::Four => 6,
            HandType::FullHouse => 5,
            HandType::Three => 4,
            HandType::TwoPairs => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

#[derive(Debug)]
struct Hand {
    bid: i32,
    cards: Vec<i32>,
    hand_type: HandType,
}

impl Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        let ord = self.hand_type.to_val().cmp(&other.hand_type.to_val());

        if ord == Ordering::Less || ord == Ordering::Greater {
            ord
        } else {
            for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                if a > b {
                    return Ordering::Greater;
                } else if a < b {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    }
}

#[derive(Debug)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(' ').collect::<Vec<&str>>();

        let bid = split[1].parse::<i32>().unwrap();
        let cards = split[0].chars().map(card_to_val).collect::<Vec<i32>>();

        let mut count_map: HashMap<i32, i32> = HashMap::new();
        for c in cards.iter() {
            if let Entry::Vacant(e) = count_map.entry(*c) {
                e.insert(1);
            } else {
                *count_map.get_mut(c).unwrap() += 1;
            }
        }

        let joker_count = count_map.remove(&1).unwrap_or(0);

        let mut count_list = count_map.values().collect::<Vec<&i32>>();
        count_list.sort();
        count_list.reverse();

        if count_list.is_empty() {
            count_list.push(&0);
        }

        let hand_type = if count_list[0] + joker_count == 5 {
            HandType::Five
        } else if count_list[0] + joker_count == 4 {
            HandType::Four
        } else if count_list[0] + joker_count == 3 {
            if count_list[1] == &2 {
                HandType::FullHouse
            } else {
                HandType::Three
            }
        } else if count_list[0] == &2 && count_list[1] == &2 {
            HandType::TwoPairs
        } else if count_list[0] + joker_count == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        };

        Ok(Hand {
            bid,
            cards,
            hand_type,
        })
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    let mut hands: Vec<Hand> = input.lines().map(|l| l.parse::<Hand>().unwrap()).collect();
    hands.sort_by(|a, b| a.cmp(b));

    let mut total = 0;
    for (i, h) in hands.iter().enumerate() {
        total += (i + 1) as i32 * h.bid;
    }
    total
}

fn card_to_val(card: char) -> i32 {
    match card {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        assert_eq!(5905, process(input));
    }

    #[test]
    fn test_parse() {
        let hand = "AQAJ5 989".parse::<Hand>().unwrap();

        assert!(matches!(hand.hand_type, HandType::Three));
    }
}
