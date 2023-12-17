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

        let count_list = count_map.values().collect::<Vec<&i32>>();
        let pair_count = count_list.iter().filter(|&&&x| x == 2).count();

        let hand_type = if count_list.contains(&&5) {
            HandType::Five
        } else if count_list.contains(&&4) {
            HandType::Four
        } else if count_list.contains(&&3) {
            if count_list.contains(&&2) {
                HandType::FullHouse
            } else {
                HandType::Three
            }
        } else if pair_count == 2 {
            HandType::TwoPairs
        } else if pair_count == 1 {
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
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
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
        assert_eq!(6440, process(input));
    }
}
