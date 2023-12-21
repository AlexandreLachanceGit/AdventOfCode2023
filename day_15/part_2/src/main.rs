use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Box {
    index: HashMap<String, usize>,
    slots: Vec<u32>,
}

impl Box {
    fn new() -> Box {
        Box {
            index: HashMap::new(),
            slots: Vec::new(),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> u32 {
    let mut boxes = vec![Box::new(); 256];

    for label in input.trim().split(',') {
        if let Some(label) = label.strip_suffix('-') {
            let label_hash = hash(label);
            let b = boxes.get_mut(label_hash).unwrap();

            if let Some(index) = b.index.remove(label) {
                b.slots.remove(index);
                for (_, val) in b.index.iter_mut() {
                    if *val > index {
                        *val -= 1;
                    }
                }
            }
        } else {
            let split: Vec<&str> = label.split('=').collect();

            let label = split[0];
            let label_hash = hash(label);
            let focal_len = split[1].parse::<u32>().unwrap();

            let b = boxes.get_mut(label_hash).unwrap();

            if let Some(index) = b.index.get(label) {
                b.slots[*index] = focal_len;
            } else {
                let index = b.slots.len();
                b.slots.push(focal_len);
                b.index.insert(label.into(), index);
            }
        }
    }

    focus_power(&boxes)
}

fn focus_power(boxes: &[Box]) -> u32 {
    let mut power = 0;
    for (i, box_) in boxes.iter().enumerate() {
        for j in 0..box_.slots.len() {
            power += (i + 1) as u32 * (j + 1) as u32 * box_.slots[j];
        }
    }
    power
}

fn hash(seq: &str) -> usize {
    let mut count = 0;
    for c in seq.chars() {
        count += c as usize;
        count *= 17;
        count %= 256;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;
        assert_eq!(145, process(input));
    }

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"));
    }
}
