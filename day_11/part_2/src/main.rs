const EXPANSION_FACTOR: usize = 1000000;

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i64 {
    let mut total = 0;

    let galaxies = get_galaxies(input);

    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            total += distance(galaxies[i], galaxies[j]);
        }
    }

    total
}

fn get_expanding(map: &[Vec<char>]) -> (Vec<usize>, Vec<usize>) {
    let expanding_ys = map
        .iter()
        .enumerate()
        .filter(|(_, line)| !line.contains(&'#'))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let mut expanding_xs = vec![];
    for i in 0..map[0].len() {
        if !map.iter().any(|r| r[i] == '#') {
            expanding_xs.push(i);
        }
    }

    (expanding_xs, expanding_ys)
}

fn get_galaxies(input: &str) -> Vec<(i64, i64)> {
    let mut map = vec![];
    for line in input.lines() {
        map.push(line.chars().collect::<Vec<char>>());
    }

    let (cols, rows) = get_expanding(&map);

    let mut galaxies = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '#' {
                let x_offset = cols.iter().filter(|&&c| c < x).count() * (EXPANSION_FACTOR - 1);
                let y_offset = rows.iter().filter(|&&c| c < y).count() * (EXPANSION_FACTOR - 1);

                galaxies.push(((x + x_offset) as i64, (y + y_offset) as i64));
            }
        }
    }
    galaxies
}

fn distance(pos_a: (i64, i64), pos_b: (i64, i64)) -> i64 {
    (pos_a.0 - pos_b.0).abs() + (pos_a.1 - pos_b.1).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        assert_eq!(1030, process(input));
    }
}
