fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|m| {
            process_map(
                &m.lines()
                    .map(|l| l.chars().collect::<Vec<char>>())
                    .collect::<Vec<Vec<char>>>(),
            )
        })
        .sum()
}

fn process_map(map: &[Vec<char>]) -> i32 {
    let res = find_vertical(map);
    if let Some(res) = res {
        return res;
    }

    100 * find_horizontal(map).unwrap()
}

fn get_col(map: &[Vec<char>], index: usize) -> Vec<char> {
    map.iter().map(|l| l[index]).collect()
}

fn find_vertical(map: &[Vec<char>]) -> Option<i32> {
    fn validate(map: &[Vec<char>], i: usize) -> bool {
        let (mut i, mut j) = (i, i + 1);

        while i > 0 && j < map[0].len() - 1 {
            i -= 1;
            j += 1;
            if get_col(map, i) != get_col(map, j) {
                return false;
            }
        }

        true
    }

    for i in 0..map[0].len() - 1 {
        if get_col(map, i) == get_col(map, i + 1) && validate(map, i) {
            return Some(i as i32 + 1);
        }
    }

    None
}

fn find_horizontal(map: &[Vec<char>]) -> Option<i32> {
    fn validate(map: &[Vec<char>], i: usize) -> bool {
        let (mut i, mut j) = (i, i + 1);

        while i > 0 && j < map.len() - 1 {
            i -= 1;
            j += 1;
            if map[i] != map[j] {
                return false;
            }
        }

        true
    }

    for i in 0..map.len() - 1 {
        if map[i] == map[i + 1] && validate(map, i) {
            return Some(i as i32 + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
        assert_eq!(405, process(input));
    }
}
