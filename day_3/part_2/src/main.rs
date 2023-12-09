fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    let mut total = 0;

    let mut map = load_map(input);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '*' {
                let numbers = get_numbers_around(&mut map, x, y);
                if numbers.len() == 2 {
                    total += numbers[0] * numbers[1];
                }
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

/*
 * x - 1, y - 1     y - 1   x + 1, y - 1
 * x - 1            x       x + 1
 * x - 1, y + 1     y + 1   x + 1, y + 1
 */
fn get_numbers_around(map: &mut [Vec<char>], x: usize, y: usize) -> Vec<i32> {
    let mut numbers = vec![];

    if x != 0 && map[y][x - 1].is_ascii_digit() {
        numbers.push(get_number_at_pos(map, x - 1, y));
    }
    if x != 0 && y != 0 && map[y - 1][x - 1].is_ascii_digit() {
        numbers.push(get_number_at_pos(map, x - 1, y - 1));
    }
    if y != 0 && map[y - 1][x].is_ascii_digit() {
        numbers.push(get_number_at_pos(map, x, y - 1));
    }
    if x < map[y].len() - 1 && y != 0 && map[y - 1][x + 1].is_ascii_digit() {
        numbers.push(get_number_at_pos(map, x + 1, y - 1));
    }
    if x < map[y].len() - 1 && map[y][x + 1].is_ascii_digit() {
        numbers.push(get_number_at_pos(map, x + 1, y));
    }
    if x < map[y].len() - 1 && y < map.len() - 1 && map[y + 1][x + 1].is_ascii_digit() {
        numbers.push(get_number_at_pos(map, x + 1, y + 1));
    }
    if y < map.len() - 1 && map[y + 1][x].is_ascii_digit() {
        numbers.push(get_number_at_pos(map, x, y + 1));
    }
    if x != 0 && y < map.len() - 1 && map[y + 1][x - 1].is_ascii_digit() {
        numbers.push(get_number_at_pos(map, x - 1, y + 1));
    }

    numbers
}

fn get_number_at_pos(map: &mut [Vec<char>], x: usize, y: usize) -> i32 {
    let mut digits = vec![map[y][x].to_digit(10).unwrap()];
    map[y][x] = '.';

    let mut pos = x;
    while pos < map[y].len() - 1 && map[y][pos + 1].is_ascii_digit() {
        digits.push(map[y][pos + 1].to_digit(10).unwrap());
        map[y][pos + 1] = '.';
        pos += 1;
    }

    if x > 0 {
        pos = x;
        while pos > 0 && map[y][pos - 1].is_ascii_digit() {
            digits.insert(0, map[y][pos - 1].to_digit(10).unwrap());
            map[y][pos - 1] = '.';
            pos -= 1;
        }
    }

    digits.iter().fold(0, |acc, &x| acc * 10 + x as i32)
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
        assert_eq!(467835, process(input));
    }

    #[test]
    fn test_get_number_at_pos() {
        let mut map = load_map(
            "
467..114..
...*......
..35..633.
......#...
617*......
.....+..85
..592.....
......755.
...$.*....
.664.598..
",
        );
        assert_eq!(467, get_number_at_pos(&mut map.clone(), 0, 0));
        assert_eq!(467, get_number_at_pos(&mut map.clone(), 1, 0));
        assert_eq!(85, get_number_at_pos(&mut map.clone(), 8, 5));
        assert_eq!(114, get_number_at_pos(&mut map, 6, 0));
    }
}
