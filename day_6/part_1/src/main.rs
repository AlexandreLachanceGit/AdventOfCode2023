#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn calculate_nb_wins(&self) -> u32 {
        // x * (time - x)>= distance

        let mut counter = 0;
        for x in 1..self.time + 1 {
            if x * (self.time - x) > self.distance {
                counter += 1;
            }
        }
        counter
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> u32 {
    load_races(input)
        .iter()
        .map(|race| race.calculate_nb_wins())
        .product()
}

fn load_races(races_str: &str) -> Vec<Race> {
    let lines = races_str.lines().collect::<Vec<&str>>();
    let times = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|t| t.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let distances = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|t| t.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    times
        .iter()
        .enumerate()
        .map(|(i, &val)| Race {
            time: val,
            distance: distances[i],
        })
        .collect::<Vec<Race>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        assert_eq!(288, process(input));
    }
}
