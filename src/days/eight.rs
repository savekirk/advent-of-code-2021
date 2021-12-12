use std::str::Chars;

pub fn part_1(lines: Vec<String>) -> i32 {
    let segments = [2, 3, 4, 7];
    lines.iter().fold(0, |mut acc, entry| {
        for output in entry.split('|').last().unwrap().trim().split(' ') {
            if segments.contains(&output.len()) {
                acc += 1;
            }
        }
        acc
    })
}

pub fn part_2(lines: Vec<String>) -> i32 {
    let mut mapping = ['x'; 7];
    lines.iter().fold(0, |mut acc, entry| {
        let mut entries = entry.split('|');
        let mut patterns: Vec<&str> = entries.next().unwrap().trim().split(' ').collect();
        patterns.sort_by(|a, b| a.len().cmp(&b.len()));
        let output = entries.next().unwrap().trim().split(' ');
        for pattern in patterns {
            let mut chars = pattern.chars();
            println!("chars => {:?}, count => {}", chars, pattern.len());
            let indexes = match pattern.len() {
                2 => vec![2, 5],
                4 => vec![1, 2, 3, 5],
                3 => vec![0, 2, 5],
                // 7 => vec![0, 1, 2, 3, 4, 5, 6],
                _ => vec![],
            };
            for i in indexes {
                let char = chars.next().unwrap();
                if !mapping.contains(&char) {
                    mapping[i] = char;
                }
                println!("current i => {}, mapping i = {}", i, mapping[i]);
            }
        }
        println!("mapping => {:?}", mapping);
        acc
    })
}
