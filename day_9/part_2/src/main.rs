fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    let mut total = 0;

    for line in input.lines() {
        total += calculate_next_number(
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );
    }

    total
}

fn calculate_next_number(history: Vec<i32>) -> i32 {
    let mut full_history = vec![history.clone()];
    full_history.extend(get_rest_of_history(&history));

    get_next_number(&full_history)
}

fn get_rest_of_history(history: &[i32]) -> Vec<Vec<i32>> {
    if history.iter().all(|n| *n == 0) {
        return vec![];
    }

    let mut new_level = vec![];
    for i in 0..history.len() - 1 {
        new_level.push(history[i + 1] - history[i]);
    }

    let children = get_rest_of_history(&new_level);
    let mut out = vec![new_level];
    out.extend(children);
    out
}

fn get_next_number(history: &[Vec<i32>]) -> i32 {
    let mut latest = 0;
    for i in (0..history.len() - 1).rev() {
        latest = history[i].first().unwrap() - latest;
    }
    latest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        assert_eq!(2, process(input));
    }
}
