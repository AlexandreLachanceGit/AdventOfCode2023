fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    tilt_north(&mut map);
    calculate_weight(&map)
}

fn calculate_weight(map: &[Vec<char>]) -> i32 {
    let mut weight = 0;
    for i in 0..map.len() {
        weight += (map.len() - i) * map[i].iter().filter(|&&c| c == 'O').count();
    }
    weight as i32
}

fn tilt_north(map: &mut [Vec<char>]) {
    for i in 0..map[0].len() {
        // Kinda bubble sort
        for j in 0..map.len() {
            for k in 0..map.len() - j - 1 {
                if map[k][i] == '.' && map[k + 1][i] == 'O' {
                    let tmp = map[k][i];
                    map[k][i] = map[k + 1][i];
                    map[k + 1][i] = tmp;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
        assert_eq!(136, process(input));
    }
}
