use std::str::FromStr;

#[derive(Debug)]
struct Line {
    springs: Vec<char>,
    groups: Vec<u32>,
}

impl Line {
    fn count_arrangements(&self) -> i32 {
        let unknown_count = self.springs.iter().filter(|&&c| c == '?').count();

        let mut arrangements = 0;
        let mut counter = 0;
        let mut bin = num_to_bin(counter, unknown_count);
        while bin.len() <= unknown_count {
            let mut bin_iter = bin.iter();
            let mut springs = vec![];
            for s in &self.springs {
                if *s == '.' || *s == '#' {
                    springs.push(*s);
                } else if *bin_iter.next().unwrap() == 0 {
                    springs.push('.');
                } else {
                    springs.push('#');
                }
            }

            let line_arrangement = Line {
                springs: springs.clone(),
                groups: self.groups.clone(),
            };

            if line_arrangement.validate() {
                arrangements += 1;
            }

            counter += 1;
            bin = num_to_bin(counter, unknown_count);
        }

        arrangements
    }

    fn validate(&self) -> bool {
        let mut valid_groups = 0;
        for (i, group) in self
            .springs
            .split(|&c| c == '.')
            .filter(|l| !l.is_empty())
            .enumerate()
        {
            if i >= self.groups.len() {
                return false;
            }
            if group.len() as u32 != self.groups[i] {
                return false;
            } else {
                valid_groups += 1;
            }
        }

        valid_groups == self.groups.len()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseLineError;

impl FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();

        Ok(Line {
            springs: parts[0].chars().collect(),
            groups: parts[1]
                .split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect(),
        })
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.parse::<Line>().unwrap())
        .fold(0, |acc, l| acc + l.count_arrangements())
}

fn num_to_bin(num: i32, width: usize) -> Vec<u32> {
    format!("{:0w$b}", num, w = width)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1"#;
        assert_eq!(21, process(input));
    }
}
