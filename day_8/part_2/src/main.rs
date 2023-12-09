use num::integer::lcm;
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug)]
struct MapStep {
    right: usize,
    left: usize,
    end: bool,
}

impl MapStep {
    fn get_dir(&self, dir: &Direction) -> usize {
        match dir {
            Direction::Right => self.right,
            Direction::Left => self.left,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i128 {
    let lines = input.lines().collect::<Vec<&str>>();
    let dirs = parse_dirs(lines[0]);
    let (start, map) = parse_map(&lines[2..]);

    let mut steps = vec![];
    for i in start {
        let mut c = 0;
        let mut current = i;
        while !map[current].end {
            let map_step: &MapStep = &map[current];
            current = map_step.get_dir(&dirs[c % dirs.len()]);
            c += 1;
        }
        steps.push(c as i128);
    }

    steps.iter().fold(1, |acc, &num| lcm(acc, num))
}

fn parse_dirs(dirs_str: &str) -> Vec<Direction> {
    dirs_str
        .trim()
        .chars()
        .map(|c| {
            if c == 'R' {
                Direction::Right
            } else {
                Direction::Left
            }
        })
        .collect::<Vec<Direction>>()
}

fn parse_map(map_lines: &[&str]) -> (Vec<usize>, Vec<MapStep>) {
    let mut start = vec![];

    let mut str_int: HashMap<&str, usize> = HashMap::new();
    for (int, line) in map_lines.iter().enumerate() {
        let str = line.split(" = ").collect::<Vec<&str>>()[0];
        str_int.insert(str, int);

        if str.chars().nth(2) == Some('A') {
            start.push(int);
        }
    }

    let mut map = vec![];

    for line in map_lines {
        let split = line.split(" = ").collect::<Vec<&str>>();
        let right_split = split[1][1..split[1].len() - 1]
            .split(", ")
            .collect::<Vec<&str>>();

        map.push(MapStep {
            right: str_int[right_split[1]],
            left: str_int[right_split[0]],
            end: split[0].chars().nth(2) == Some('Z'),
        });
    }

    (start, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
        assert_eq!(6, process(input));
    }
}
