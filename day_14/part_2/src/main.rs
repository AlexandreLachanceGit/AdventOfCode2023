fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut maps: Vec<Vec<Vec<char>>> = vec![];

    let mut i = 0;
    loop {
        spin(&mut map);
        if let Some(pos) = maps.iter().position(|m| *m == map) {
            let index = (1000000000 - pos - 1) % (i - pos);
            let w = calculate_weight(&maps[index + pos]);
            return w;
        } else {
            maps.push(map.clone());
        }
        i += 1;
    }
}

fn calculate_weight(map: &[Vec<char>]) -> i32 {
    let mut weight = 0;
    for i in 0..map.len() {
        weight += (map.len() - i) * map[i].iter().filter(|&&c| c == 'O').count();
    }
    weight as i32
}

fn spin(map: &mut [Vec<char>]) {
    tilt_north(map);
    tilt_west(map);
    tilt_south(map);
    tilt_east(map);
}

fn tilt_north(map: &mut [Vec<char>]) {
    for i in 0..map[0].len() {
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

fn tilt_west(map: &mut [Vec<char>]) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            for k in 0..map[0].len() - j - 1 {
                if map[i][k] == '.' && map[i][k + 1] == 'O' {
                    map[i].swap(k, k + 1);
                }
            }
        }
    }
}

fn tilt_south(map: &mut [Vec<char>]) {
    for i in 0..map[0].len() {
        for j in 0..map.len() {
            for k in 0..map.len() - j - 1 {
                if map[k][i] == 'O' && map[k + 1][i] == '.' {
                    let tmp = map[k][i];
                    map[k][i] = map[k + 1][i];
                    map[k + 1][i] = tmp;
                }
            }
        }
    }
}

fn tilt_east(map: &mut [Vec<char>]) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            for k in 0..map[0].len() - j - 1 {
                if map[i][k] == 'O' && map[i][k + 1] == '.' {
                    map[i].swap(k, k + 1);
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
        assert_eq!(64, process(input));
    }
}
