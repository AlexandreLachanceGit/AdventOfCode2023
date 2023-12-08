fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    let mut total = 0;

    let map = load_map(input);

    let mut number: Option<i32> = None;
    let mut is_symbol_around = false;
    for y in 0..map.len() {
        if let Some(n) = number {
            if is_symbol_around {
                total += n;
            }
            number = None;
        }
        is_symbol_around = false;
        for x in 0..map[y].len() {
            let c = map[y][x];

            if c.is_ascii_digit() {
                let current_digit = c.to_digit(10).unwrap() as i32;
                if number.is_some() {
                    let mut n = number.unwrap();
                    n = n * 10 + current_digit;
                    number = Some(n);
                } else {
                    number = Some(current_digit);
                }
                if symbol_around(&map, x, y) {
                    is_symbol_around = true;
                }
            } else {
                if let Some(n) = number {
                    if is_symbol_around {
                        total += n;
                    }
                }
                number = None;
                is_symbol_around = false;
            }
        }
    }

    total
}

fn load_map(map_str: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in map_str.lines() {
        let trimmed_line = line.trim();
        if !trimmed_line.is_empty() {
            map.push(trimmed_line.chars().collect::<Vec<char>>());
        }
    }
    map
}

fn is_symbol(c: char) -> bool {
    if c == '.' {
        return false;
    }
    c.is_ascii_punctuation()
}

/*
 * x - 1, y - 1     y - 1   x + 1, y - 1
 * x - 1            x       x + 1
 * x - 1, y + 1     y + 1   x + 1, y + 1
 */
fn symbol_around(map: &[Vec<char>], x: usize, y: usize) -> bool {
    (x != 0 && is_symbol(map[y][x - 1]))
        || (x != 0 && y != 0 && is_symbol(map[y - 1][x - 1]))
        || (y != 0 && is_symbol(map[y - 1][x]))
        || (x < map[y].len() - 1 && y != 0 && is_symbol(map[y - 1][x + 1]))
        || (x < map[y].len() - 1 && is_symbol(map[y][x + 1]))
        || (x < map[y].len() - 1 && y < map.len() - 1 && is_symbol(map[y + 1][x + 1]))
        || (y < map.len() - 1 && is_symbol(map[y + 1][x]))
        || (x != 0 && y < map.len() - 1 && is_symbol(map[y + 1][x - 1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
        "#;
        assert_eq!(4361, process(input));
    }

    #[test]
    fn test_symbol_around() {
        let map = load_map(
            "
                              ...
                              .11
                              ...
                           ",
        );
        assert!(!symbol_around(&map, 2, 1));
        let map = load_map(
            "
                              ...
                              !1.
                              ...
                           ",
        );
        assert!(symbol_around(&map, 1, 1));
    }
}
