fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> i32 {
    let mut total = 0;

    let map = expand(input);
    let galaxies = get_galaxies(&map);

    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            total += distance(galaxies[i], galaxies[j]);
        }
    }

    total
}

fn expand(input: &str) -> Vec<Vec<char>> {
    let mut lines = vec![];
    for line in input.lines() {
        lines.push(line.chars().collect::<Vec<char>>());
    }

    let expanding_ys = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| !line.contains(&'#'))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    for (i, y) in expanding_ys.iter().enumerate() {
        lines.insert(
            y + i,
            ".".repeat(lines[0].len()).chars().collect::<Vec<char>>(),
        );
    }

    let mut expanding_xs = vec![];
    for i in 0..lines[0].len() {
        if !lines.iter().any(|r| r[i] == '#') {
            expanding_xs.push(i);
        }
    }

    for (i, x) in expanding_xs.iter().enumerate() {
        for l in &mut lines {
            l.insert(x + i, '.');
        }
    }

    lines
}

fn get_galaxies(map: &[Vec<char>]) -> Vec<(i32, i32)> {
    let mut galaxies = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '#' {
                galaxies.push((x as i32, y as i32));
            }
        }
    }
    galaxies
}

fn distance(pos_a: (i32, i32), pos_b: (i32, i32)) -> i32 {
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
        assert_eq!(374, process(input));
    }
}
