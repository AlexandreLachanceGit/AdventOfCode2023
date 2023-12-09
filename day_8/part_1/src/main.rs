use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug)]
struct MapStep {
    right: String,
    left: String,
}

impl MapStep {
    fn get_dir(&self, dir: &Direction) -> String {
        match dir {
            Direction::Right => self.right.clone(),
            Direction::Left => self.left.clone(),
        }
    }
}

#[derive(Debug)]
struct MapStepError;

impl FromStr for MapStep {
    type Err = MapStepError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s[1..s.len() - 1].split(", ").collect::<Vec<&str>>();
        Ok(MapStep {
            left: split[0].to_owned(),
            right: split[1].to_owned(),
        })
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let dirs = parse_dirs(lines[0]);
    let map = parse_map(&lines[2..]);

    calculate_steps(&map, "AAA", &dirs, 0)
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

fn parse_map(map_lines: &[&str]) -> HashMap<String, MapStep> {
    let mut map = HashMap::new();

    for line in map_lines {
        let split = line.split(" = ").collect::<Vec<&str>>();
        map.insert(split[0].to_string(), split[1].parse::<MapStep>().unwrap());
    }

    map
}

fn calculate_steps(
    map: &HashMap<String, MapStep>,
    current_step: &str,
    dirs: &[Direction],
    dir_i: usize,
) -> i32 {
    if current_step == "ZZZ" {
        return 0;
    }
    let dir_i = if dir_i == dirs.len() { 0 } else { dir_i };

    let next_step = map[current_step].get_dir(&dirs[dir_i]);
    1 + calculate_steps(map, &next_step, dirs, dir_i + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
        assert_eq!(2, process(input));
    }

    #[test]
    fn test_process_2() {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
        assert_eq!(6, process(input));
    }
}
