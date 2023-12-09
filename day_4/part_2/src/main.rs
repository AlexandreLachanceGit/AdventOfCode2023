use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    let cards = load_cards(input);
    let mut card_count = vec![1; cards.len()];

    for i in 0..cards.len() {
        for j in 1..=cards[i] {
            card_count[i + j as usize] += card_count[i];
        }
    }

    card_count.iter().sum()
}

fn load_cards(cards_str: &str) -> Vec<i32> {
    let mut cards = Vec::new();
    for line in cards_str.lines() {
        let card = line.split(':').collect::<Vec<&str>>();

        let card_numbers = card[1].split('|').collect::<Vec<&str>>();
        let winning_numbers: HashSet<i32> = HashSet::from_iter(
            card_numbers[0]
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap()),
        );

        let numbers = card_numbers[1]
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let nb_matching = count_nb_matching(&winning_numbers, &numbers);
        cards.push(nb_matching);
    }
    cards
}

fn count_nb_matching(winning_numbers: &HashSet<i32>, numbers: &[i32]) -> i32 {
    let mut matching_count = 0;

    for n in numbers {
        if winning_numbers.contains(n) {
            matching_count += 1;
        }
    }

    matching_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(30, process(input));
    }
}
