use std::collections::HashSet;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();
    let mut used_dir_map: Vec<Vec<HashSet<Direction>>> = vec![
        vec![
            HashSet::from_iter(vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right
            ]);
            map[0].len()
        ];
        map.len()
    ];
    let mut energized_map: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];

    beam(
        &map,
        &mut used_dir_map,
        &mut energized_map,
        (0, 0),
        Direction::Right,
    );

    energized_map
        .iter()
        .map(|l| l.iter().filter(|&&c| c).count())
        .sum()
}

fn inc_beam(
    map: &[Vec<char>],
    used_dir_map: &mut [Vec<HashSet<Direction>>],
    energized_map: &mut [Vec<bool>],
    pos: (usize, usize),
    dir: Direction,
) {
    if let Some(new_pos) = increment_pos(map, pos, dir) {
        beam(map, used_dir_map, energized_map, new_pos, dir);
    }
}

fn beam(
    map: &[Vec<char>],
    used_dir_map: &mut [Vec<HashSet<Direction>>],
    energized_map: &mut [Vec<bool>],
    pos: (usize, usize),
    dir: Direction,
) {
    energized_map[pos.1][pos.0] = true;
    if !used_dir_map[pos.1][pos.0].remove(&dir) {
        return;
    };

    match map[pos.1][pos.0] {
        '.' => {
            inc_beam(map, used_dir_map, energized_map, pos, dir);
        }
        '|' => {
            if matches!(dir, Direction::Right | Direction::Left) {
                inc_beam(map, used_dir_map, energized_map, pos, Direction::Up);
                inc_beam(map, used_dir_map, energized_map, pos, Direction::Down);
            } else {
                inc_beam(map, used_dir_map, energized_map, pos, dir);
            }
        }
        '-' => {
            if matches!(dir, Direction::Up | Direction::Down) {
                inc_beam(map, used_dir_map, energized_map, pos, Direction::Left);
                inc_beam(map, used_dir_map, energized_map, pos, Direction::Right);
            } else {
                inc_beam(map, used_dir_map, energized_map, pos, dir);
            }
        }
        '/' => match dir {
            Direction::Down => {
                inc_beam(map, used_dir_map, energized_map, pos, Direction::Left);
            }
            Direction::Up => {
                inc_beam(map, used_dir_map, energized_map, pos, Direction::Right);
            }
            Direction::Right => {
                inc_beam(map, used_dir_map, energized_map, pos, Direction::Up);
            }
            Direction::Left => {
                inc_beam(map, used_dir_map, energized_map, pos, Direction::Down);
            }
        },
        '\\' => match dir {
            Direction::Down => {
                inc_beam(map, used_dir_map, energized_map, pos, Direction::Right);
            }
            Direction::Up => {
                inc_beam(map, used_dir_map, energized_map, pos, Direction::Left);
            }
            Direction::Right => {
                inc_beam(map, used_dir_map, energized_map, pos, Direction::Down);
            }
            Direction::Left => {
                inc_beam(map, used_dir_map, energized_map, pos, Direction::Up);
            }
        },
        _ => unimplemented!(),
    }
}

fn increment_pos(map: &[Vec<char>], pos: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
    match dir {
        Direction::Up => {
            if pos.1 == 0 {
                None
            } else {
                Some((pos.0, pos.1 - 1))
            }
        }
        Direction::Down => {
            if pos.1 + 1 >= map.len() {
                None
            } else {
                Some((pos.0, pos.1 + 1))
            }
        }
        Direction::Left => {
            if pos.0 == 0 {
                None
            } else {
                Some((pos.0 - 1, pos.1))
            }
        }
        Direction::Right => {
            if pos.0 + 1 >= map[0].len() {
                None
            } else {
                Some((pos.0 + 1, pos.1))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!(46, process(input));
    }
}
