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

fn diff_col(map: &[Vec<char>], i: usize, j: usize) -> i32 {
    let mut n = 0;
    for (a, b) in get_col(map, i).iter().zip(get_col(map, j).iter()) {
        if a != b {
            n += 1;
        }
    }
    n
}

fn find_vertical(map: &[Vec<char>]) -> Option<i32> {
    fn validate(map: &[Vec<char>], i: usize) -> bool {
        let (mut i, mut j) = (i, i + 1);
        let mut smudge_count = diff_col(map, i, j);

        while i > 0 && j < map[0].len() - 1 {
            i -= 1;
            j += 1;
            smudge_count += diff_col(map, i, j);
            if smudge_count > 1 {
                return false;
            }
        }

        smudge_count == 1
    }

    for i in 0..map[0].len() - 1 {
        if (get_col(map, i) == get_col(map, i + 1) || diff_col(map, i, i + 1) == 1)
            && validate(map, i)
        {
            return Some(i as i32 + 1);
        }
    }

    None
}

fn find_horizontal(map: &[Vec<char>]) -> Option<i32> {
    fn validate(map: &[Vec<char>], i: usize) -> bool {
        let (mut i, mut j) = (i, i + 1);
        let mut smudge_count = diff_row(map, i, j);

        while i > 0 && j < map.len() - 1 {
            i -= 1;
            j += 1;
            smudge_count += diff_row(map, i, j);
            if smudge_count > 1 {
                return false;
            }
        }

        smudge_count == 1
    }

    for i in 0..map.len() - 1 {
        if (map[i] == map[i + 1] || diff_row(map, i, i + 1) == 1) && validate(map, i) {
            return Some(i as i32 + 1);
        }
    }

    None
}

fn diff_row(map: &[Vec<char>], i: usize, j: usize) -> i32 {
    let mut n = 0;
    for (a, b) in map[i].iter().zip(map[j].iter()) {
        if a != b {
            n += 1;
        }
    }
    n
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
        assert_eq!(400, process(input));
    }
}
