fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        let line_nb_str = match numbers.len() {
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
        };

        total += line_nb_str.parse::<i32>().unwrap();
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "#;
        assert_eq!(142, process(input));
    }
}
