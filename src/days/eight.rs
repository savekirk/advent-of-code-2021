use std::collections::{HashMap, HashSet};

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

fn intersection(sets: Vec<HashSet<char>>) -> HashSet<char> {
    sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
        acc.intersection(hs).cloned().collect()
    })
}

fn difference(sets: Vec<HashSet<char>>) -> HashSet<char> {
    sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
        acc.difference(hs).cloned().collect()
    })
}

fn appear_count(sets: Vec<HashSet<char>>, count: i8) -> HashSet<char> {
    let mut char_map: HashMap<char, i8> = HashMap::new();
    for chars in sets {
        for c in chars {
            char_map.entry(c).and_modify(|cc| *cc += 1).or_insert(1);
        }
    }

    char_map.iter().fold(HashSet::new(), |mut acc, (c, cc)| {
        if *cc == count {
            acc.insert(*c);
        }
        acc
    })
}

fn decode_output(&mapping: &[&char; 7], output: Vec<char>) -> char {
    match output.len() {
        2 => '1',
        3 => '7',
        4 => '4',
        7 => '8',
        _ => {
            let mut pos = output.iter().fold(vec![], |mut acc, char| {
                acc.push(mapping.iter().position(|&c| *c == *char).unwrap());
                acc
            });
            pos.sort();
            if pos == vec![0, 2, 3, 4, 6] {
                return '2';
            } else if pos == vec![0, 2, 3, 5, 6] {
                return '3';
            } else if pos == vec![0, 1, 3, 5, 6] {
                return '5';
            } else if pos == vec![0, 1, 3, 4, 5, 6] {
                return '6';
            } else if pos == vec![0, 1, 2, 3, 5, 6] {
                return '9';
            } else if pos == vec![0, 1, 2, 4, 5, 6] {
                return '0';
            }
            return '0';
        }
    }
}

pub fn part_2(lines: Vec<String>) -> i32 {
    lines.iter().fold(0, |mut acc, entry| {
        let mut len_2 = HashSet::new();
        let mut len_3 = HashSet::new();
        let mut len_4 = HashSet::new();
        let mut len_5 = vec![];
        let mut len_6 = vec![];
        let mut len_7 = HashSet::new();

        let mut entries = entry.split('|');
        let mut patterns: Vec<&str> = entries.next().unwrap().trim().split(' ').collect();
        patterns.sort_by(|a, b| a.len().cmp(&b.len()));
        let outputs: Vec<&str> = entries.next().unwrap().trim().split(' ').collect();
        for pattern in patterns {
            let chars = pattern.chars().collect::<HashSet<char>>();
            match pattern.len() {
                2 => len_2 = chars,
                3 => len_3 = chars,
                4 => len_4 = chars,
                5 => len_5.push(chars),
                6 => len_6.push(chars),
                7 => len_7 = chars,
                _ => {}
            };
        }

        let appear_2_in_5_6 = intersection(vec![
            appear_count(len_5.clone(), 2),
            appear_count(len_6.clone(), 2),
        ]);

        let in_0 = intersection(
            vec![
                vec![len_3.clone(), len_7.clone()],
                len_5.clone(),
                len_6.clone(),
            ]
            .concat(),
        );

        let in_1 = intersection(vec![
            appear_count(len_5.clone(), 1),
            intersection(len_6.clone()),
        ]);

        let in_2 = intersection(vec![
            len_2.clone(),
            len_3.clone(),
            len_4.clone(),
            len_7.clone(),
            appear_2_in_5_6,
        ]);

        let appear_3_in_5_2_6 = intersection(vec![
            intersection(len_5.clone()),
            appear_count(len_6.clone(), 2),
        ]);

        let in_3 = intersection(vec![len_4.clone(), len_7.clone(), appear_3_in_5_2_6]);

        let in_4 = intersection(vec![
            appear_count(len_5.clone(), 1),
            appear_count(len_6.clone(), 2),
        ]);

        let appear_2_in_5_3_6 = intersection(vec![
            appear_count(len_5.clone(), 2),
            intersection(len_6.clone()),
        ]);

        let in_5 = intersection(vec![
            len_2.clone(),
            len_3.clone(),
            len_4.clone(),
            len_7.clone(),
            appear_2_in_5_3_6,
        ]);

        let in_6 = difference(vec![
            intersection([len_5.clone(), len_6.clone()].concat()),
            len_3.clone(),
        ]);

        let mapping = [
            in_0.iter().last().unwrap(),
            in_1.iter().last().unwrap(),
            in_2.iter().last().unwrap(),
            in_3.iter().last().unwrap(),
            in_4.iter().last().unwrap(),
            in_5.iter().last().unwrap(),
            in_6.iter().last().unwrap(),
        ];

        let mut digits = String::new();
        for output in outputs {
            let chars = output.chars().collect::<Vec<char>>();
            digits.push(decode_output(&mapping, chars));
        }
        acc += digits.parse::<i32>().unwrap();
        acc
    })
}
