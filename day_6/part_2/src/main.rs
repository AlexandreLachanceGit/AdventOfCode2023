#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn calculate_nb_wins(&self) -> u64 {
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

fn process(input: &str) -> u64 {
    load_race(input).calculate_nb_wins()
}

fn load_race(races_str: &str) -> Race {
    let lines = races_str.lines().collect::<Vec<&str>>();
    let time = lines[0].split_whitespace().collect::<Vec<&str>>()[1..]
        .join("")
        .parse::<u64>()
        .unwrap();
    let distance = lines[1].split_whitespace().collect::<Vec<&str>>()[1..]
        .join("")
        .parse::<u64>()
        .unwrap();

    Race { time, distance }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        assert_eq!(71503, process(input));
    }
}
