enum Position {
    Pipe { a: (i8, i8), b: (i8, i8) },
    Ground,
    Start,
}

#[rustfmt::skip]
fn parse_position(c: char) -> Position {
    match c {
        '.' => Position::Ground,
        'S' => Position::Start,
        '|' => Position::Pipe { a: (0, -1), b: (0, 1) },
        '-' => Position::Pipe { a: (-1, 0), b: (1, 0) },
        'L' => Position::Pipe { a: (0, -1), b: (1, 0) },
        'J' => Position::Pipe { a: (0, -1), b: (-1, 0) },
        '7' => Position::Pipe { a: (0, 1), b: (-1, 0) },
        'F' => Position::Pipe { a: (0, 1), b: (1, 0) },
        _ => unreachable!(),
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    let (map, start) = load_map(input);

    let mut current_pos = (start, start);
    let mut current_dirs = get_start_dirs(&map, start);
    let mut counter = 1;

    current_pos = (
        move_dir(current_pos.0, current_dirs.0),
        move_dir(current_pos.1, current_dirs.1),
    );
    current_dirs = (
        get_next_dir(&map, current_dirs.0, current_pos.0),
        get_next_dir(&map, current_dirs.1, current_pos.1),
    );

    while current_pos.0 != current_pos.1 {
        current_pos = (
            move_dir(current_pos.0, current_dirs.0),
            move_dir(current_pos.1, current_dirs.1),
        );
        current_dirs = (
            get_next_dir(&map, current_dirs.0, current_pos.0),
            get_next_dir(&map, current_dirs.1, current_pos.1),
        );

        counter += 1;
    }

    counter
}

fn get_next_dir(map: &[Vec<Position>], current_dir: (i8, i8), pos: (usize, usize)) -> (i8, i8) {
    let pos = &map[pos.1][pos.0];

    if let Position::Pipe { a, b } = pos {
        if *a == reverse_dir(current_dir) {
            *b
        } else {
            *a
        }
    } else {
        unreachable!()
    }
}

fn move_dir(pos: (usize, usize), dir: (i8, i8)) -> (usize, usize) {
    (
        (pos.0 as i32 + dir.0 as i32) as usize,
        (pos.1 as i32 + dir.1 as i32) as usize,
    )
}

fn get_start_dirs(map: &[Vec<Position>], start: (usize, usize)) -> ((i8, i8), (i8, i8)) {
    let possible_dirs: [(i8, i8); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut start_dirs = vec![];

    for dir in possible_dirs {
        let maybe_start_pos = move_dir(start, dir);

        if maybe_start_pos.0 < map[0].len() && maybe_start_pos.1 < map.len() {
            let maybe_start = &map[maybe_start_pos.1][maybe_start_pos.0];

            if let Position::Pipe { a, b } = maybe_start {
                if *a == reverse_dir(dir) || *b == reverse_dir(dir) {
                    start_dirs.push(dir);
                }
            }
        }
    }

    assert_eq!(2, start_dirs.len());
    (start_dirs[0], start_dirs[1])
}

fn reverse_dir(dir: (i8, i8)) -> (i8, i8) {
    (-dir.0, -dir.1)
}

fn load_map(map_str: &str) -> (Vec<Vec<Position>>, (usize, usize)) {
    let mut map = vec![];
    let mut start: (usize, usize) = (0, 0);

    for (y, line) in map_str.lines().enumerate() {
        let mut line_vec = vec![];
        for (x, char) in line.chars().enumerate() {
            let pos = parse_position(char);
            if matches!(pos, Position::Start) {
                start = (x, y);
            }
            line_vec.push(parse_position(char));
        }
        map.push(line_vec);
    }

    (map, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#".....
.S-7.
.|.|.
.L-J.
....."#;
        assert_eq!(4, process(input));
    }

    #[test]
    fn test_process_2() {
        let input = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
        assert_eq!(8, process(input));
    }
}
