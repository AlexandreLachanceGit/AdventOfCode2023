use std::{ops::Range, str::FromStr};

struct Step {
    maps: Vec<Map>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseStepError;

impl FromStr for Step {
    type Err = ParseStepError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Step {
            maps: s
                .lines()
                .skip(1)
                .map(|m| m.parse::<Map>().unwrap())
                .collect(),
        })
    }
}

impl Step {
    fn translate(&self, value: i64) -> i64 {
        for m in &self.maps {
            if m.range.contains(&value) {
                return value + m.distance;
            }
        }

        value
    }
}

struct Map {
    range: Range<i64>,
    distance: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMapError;

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        Ok(Map {
            range: numbers[1]..numbers[1] + numbers[2],
            distance: numbers[0] - numbers[1],
        })
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i64 {
    let seeds = input.lines().collect::<Vec<&str>>()[0]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let steps: Vec<Step> = input
        .split("\n\n")
        .map(|s| s.parse::<Step>().unwrap())
        .collect();

    let mut results: Vec<i64> = Vec::new();

    for seed in seeds {
        let mut val = seed;
        for step in &steps {
            val = step.translate(val);
        }
        results.push(val);
    }

    *results.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
        assert_eq!(35, process(input));
    }
}
