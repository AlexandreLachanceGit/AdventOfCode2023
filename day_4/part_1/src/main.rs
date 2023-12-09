use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let card_numbers = line.split(':').collect::<Vec<&str>>()[1]
            .split('|')
            .collect::<Vec<&str>>();
        let winning_numbers: HashSet<i32> = HashSet::from_iter(
            card_numbers[0]
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap()),
        );

        let numbers = card_numbers[1]
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        total += calculate_card_points(&winning_numbers, &numbers);
    }

    total
}

fn calculate_card_points(winning_numbers: &HashSet<i32>, numbers: &[i32]) -> i32 {
    let mut points = 0;

    for n in numbers {
        if winning_numbers.contains(n) {
            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }
    }

    points
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
        assert_eq!(13, process(input));
    }
}
