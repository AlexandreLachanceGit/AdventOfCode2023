const WORD_DIGIT: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = parse_to_list(line);

        let line_nb = match numbers.len() {
            0 => {
                // No digit in line
                continue;
            }
            1 => {
                format!("{}{}", numbers[0], numbers[0])
            }
            _ => {
                format!("{}{}", numbers[0], numbers.last().unwrap())
            }
        }
        .parse::<i32>()
        .unwrap();

        total += line_nb;
    }

    total
}

fn parse_to_list(line: &str) -> Vec<u32> {
    if line.is_empty() {
        return Vec::new();
    }

    let first_char: char = line.chars().next().unwrap();
    if let Some(digit) = first_char.to_digit(10) {
        let mut new_vec = vec![digit];
        new_vec.extend(parse_to_list(&line[1..]));
        return new_vec;
    } else {
        for (word, digit) in WORD_DIGIT {
            if line.starts_with(word) {
                let mut new_vec = vec![digit];
                new_vec.extend(parse_to_list(&line[1..]));
                return new_vec;
            }
        }
    }

    parse_to_list(&line[1..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "#;
        assert_eq!(281 + 142, process(input));
    }

    #[test]
    fn test_process_2() {
        let input = r#"
            two1nine998
        "#;
        assert_eq!(28, process(input));
    }
}
