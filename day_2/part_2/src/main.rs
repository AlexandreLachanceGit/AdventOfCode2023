use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let split_id_game = line.split(':').collect::<Vec<&str>>();
        if split_id_game.len() > 1 {
            total += calculate_game_power(split_id_game[1]);
        }
    }
    total
}

fn calculate_game_power(game: &str) -> i32 {
    let mut max_color = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    for set in game.split(';') {
        for count_color in set.split(',') {
            let count_color_split: Vec<&str> = count_color.trim().split(' ').collect();
            let count = count_color_split[0].parse::<i32>().unwrap();
            let color = count_color_split[1];

            if count > *max_color.get(color).unwrap() {
                *max_color.get_mut(color).unwrap() = count;
            }
        }
    }

    max_color.get("red").unwrap() * max_color.get("green").unwrap() * max_color.get("blue").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        assert_eq!(2286, process(input));
    }
}
