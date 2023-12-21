use std::{
    cmp::{max, min},
    ops::RangeInclusive,
    str::FromStr,
};

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
    fn translate(&self, range: RangeInclusive<i64>) -> Vec<RangeInclusive<i64>> {
        let mut ranges = vec![];

        let mut t = vec![range];
        while let Some(r) = t.pop() {
            for m in &self.maps {
                if let Some((overlap, outside)) = get_overlap(&r, &m.range) {
                    ranges.push(overlap.start() + m.distance..=overlap.end() + m.distance);
                    t.extend(outside);
                }
            }
        }

        ranges
    }
}

fn get_overlap(
    range_1: &RangeInclusive<i64>,
    range_2: &RangeInclusive<i64>,
) -> Option<(RangeInclusive<i64>, Vec<RangeInclusive<i64>>)> {
    let start = max(range_1.start(), range_2.start());
    let end = min(range_1.end(), range_2.end());

    let mut outside = vec![];

    if range_1.start() < start {
        outside.push(*range_1.start()..=*start);
    }
    if end < range_1.end() {
        outside.push(*end..=*range_1.end());
    }

    if start < end {
        Some((*start..=*end, outside))
    } else {
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    range: RangeInclusive<i64>,
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
            range: numbers[1]..=numbers[1] + numbers[2] - 1,
            distance: numbers[0] - numbers[1],
        })
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i64 {
    let seeds_range_parts = input.lines().collect::<Vec<&str>>()[0]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut seed_ranges: Vec<RangeInclusive<i64>> = vec![];
    for i in (0..seeds_range_parts.len()).step_by(2) {
        seed_ranges
            .push(seeds_range_parts[i]..=seeds_range_parts[i] + seeds_range_parts[i + 1] - 1);
    }

    let steps: Vec<Step> = input
        .split("\n\n")
        .skip(1)
        .map(|s| s.parse::<Step>().unwrap())
        .collect();

    for step in &steps {
        let mut new = vec![];
        for r in seed_ranges {
            new.extend(step.translate(r));
        }
        seed_ranges = new;
    }

    let mut results: Vec<i64> = Vec::new();
    for r in seed_ranges {
        results.push(*r.start());
    }

    results.sort();
    // println!("{:?}", &results[..20]);
    results[0]
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
        assert_eq!(45, process(input));
    }

    #[test]
    fn test_process_2() {
        let input = r#"seeds: 1 10

seed-to-soil map:
30 1 7"#;
        assert_eq!(46, process(input));
    }

    #[test]
    fn test_missing_ranges() {
        assert_eq!(
            vec![4..=6, 10..=12],
            get_missing_ranges(&(1..=12), &[1..=3, 7..=9])
        );
        assert_eq!(
            vec![4..=4],
            get_missing_ranges(&(1..=10), &[1..=3, 5..=6, 7..=10])
        );
    }
}
