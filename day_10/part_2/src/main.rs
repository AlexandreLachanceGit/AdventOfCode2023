#[derive(PartialEq, Clone, Copy, Debug)]
enum Position {
    Pipe { a: (i8, i8), b: (i8, i8) },
    Ground,
    Start,
    Border,
    Outside,
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
    let map = gen_border_map(input);

    let mut expanded = expand_map(&map, input);
    let dimensions = (expanded[0].len(), expanded.len());

    // Fill from Outline
    for i in 0..dimensions.0 {
        fill(&mut expanded, i as i32, 0);
        fill(&mut expanded, i as i32, dimensions.1 as i32 - 1);
    }

    for i in 0..dimensions.1 {
        fill(&mut expanded, 0, i as i32);
        fill(&mut expanded, dimensions.0 as i32 - 1, i as i32);
    }

    // Count
    let mut count = 0;
    for i in (1..expanded.len()).step_by(3) {
        for j in (1..expanded[0].len()).step_by(3) {
            if expanded[i][j] != Position::Border && expanded[i][j] != Position::Outside {
                count += 1;
            }
        }
    }

    count
}

fn expand_map(map: &[Vec<Position>], input: &str) -> Vec<Vec<Position>> {
    let map_char = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut expanded = vec![vec![Position::Ground; map[0].len() * 3]; map.len() * 3];

    for i in 0..map_char.len() {
        for j in 0..map_char[0].len() {
            if map[i][j] == Position::Border || map[i][j] == Position::Start {
                let part = match map_char[i][j] {
                    '|' => [
                        [Position::Ground, Position::Border, Position::Ground],
                        [Position::Ground, Position::Border, Position::Ground],
                        [Position::Ground, Position::Border, Position::Ground],
                    ],
                    '-' => [
                        [Position::Ground, Position::Ground, Position::Ground],
                        [Position::Border, Position::Border, Position::Border],
                        [Position::Ground, Position::Ground, Position::Ground],
                    ],
                    'L' => [
                        [Position::Ground, Position::Border, Position::Ground],
                        [Position::Ground, Position::Border, Position::Border],
                        [Position::Ground, Position::Ground, Position::Ground],
                    ],
                    'J' => [
                        [Position::Ground, Position::Border, Position::Ground],
                        [Position::Border, Position::Border, Position::Ground],
                        [Position::Ground, Position::Ground, Position::Ground],
                    ],
                    '7' => [
                        [Position::Ground, Position::Ground, Position::Ground],
                        [Position::Border, Position::Border, Position::Ground],
                        [Position::Ground, Position::Border, Position::Ground],
                    ],
                    'F' => [
                        [Position::Ground, Position::Ground, Position::Ground],
                        [Position::Ground, Position::Border, Position::Border],
                        [Position::Ground, Position::Border, Position::Ground],
                    ],
                    'S' => [
                        [Position::Border, Position::Border, Position::Border],
                        [Position::Border, Position::Border, Position::Border],
                        [Position::Border, Position::Border, Position::Border],
                    ],
                    _ => unreachable!(),
                };

                for k in 0..3 {
                    for l in 0..3 {
                        expanded[i * 3 + l][j * 3 + k] = part[l][k];
                    }
                }
            }
        }
    }

    expanded
}

fn fill(map: &mut [Vec<Position>], x: i32, y: i32) {
    let mut stack = Vec::new();
    stack.push((x, y));

    while let Some((cur_x, cur_y)) = stack.pop() {
        if cur_y < 0 || cur_y >= map.len() as i32 || cur_x < 0 || cur_x >= map[0].len() as i32 {
            continue;
        }

        if map[cur_y as usize][cur_x as usize] == Position::Border
            || map[cur_y as usize][cur_x as usize] == Position::Outside
        {
            continue;
        }

        map[cur_y as usize][cur_x as usize] = Position::Outside;

        for &(new_y, new_x) in &[
            (cur_y - 1, cur_x - 1),
            (cur_y - 1, cur_x),
            (cur_y - 1, cur_x + 1),
            (cur_y, cur_x - 1),
            (cur_y, cur_x + 1),
            (cur_y + 1, cur_x - 1),
            (cur_y + 1, cur_x),
            (cur_y + 1, cur_x + 1),
        ] {
            stack.push((new_x, new_y));
        }
    }
}

fn gen_border_map(input: &str) -> Vec<Vec<Position>> {
    let (mut map, start) = load_map(input);

    let mut current_pos = (start, start);
    let mut current_dirs = get_start_dirs(&map, start);

    current_pos = (
        move_dir(current_pos.0, current_dirs.0),
        move_dir(current_pos.1, current_dirs.1),
    );
    current_dirs = (
        get_next_dir(&map, current_dirs.0, current_pos.0),
        get_next_dir(&map, current_dirs.1, current_pos.1),
    );
    map[current_pos.0 .1][current_pos.0 .0] = Position::Border;
    map[current_pos.1 .1][current_pos.1 .0] = Position::Border;

    while current_pos.0 != current_pos.1 {
        current_pos = (
            move_dir(current_pos.0, current_dirs.0),
            move_dir(current_pos.1, current_dirs.1),
        );
        current_dirs = (
            get_next_dir(&map, current_dirs.0, current_pos.0),
            get_next_dir(&map, current_dirs.1, current_pos.1),
        );

        map[current_pos.0 .1][current_pos.0 .0] = Position::Border;
        map[current_pos.1 .1][current_pos.1 .0] = Position::Border;
    }

    map
}

fn print_map(map: &[Vec<Position>]) {
    for row in map {
        for pos in row {
            let char = match pos {
                Position::Border => 'B',
                Position::Outside => 'O',
                _ => '.',
            };
            print!("{char}");
        }
        println!();
    }
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
        let input = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;
        assert_eq!(4, process(input));
    }

    #[test]
    fn test_process_2() {
        let input = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;
        assert_eq!(8, process(input));
    }
    #[test]
    fn test_process_3() {
        let input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;
        assert_eq!(10, process(input));
    }
}
