fn main() {
    let input = include_str!("../input.txt");
    println!("Answer: {}", process(input));
}

fn process(input: &str) -> u64 {
    input.trim().split(',').map(hash).sum()
}

fn hash(seq: &str) -> u64 {
    let mut count = 0;
    for c in seq.chars() {
        count += c as u64;
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
        assert_eq!(1320, process(input));
    }

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"));
    }
}
