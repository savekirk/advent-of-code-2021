use std::vec;

use crate::days::utils::line_to_chars;

enum Chunk {
    Open(char),
    Close(char),
}

impl Chunk {
    fn parse(c: char) -> Option<Self> {
        match c {
            '(' | '[' | '{' | '<' => Some(Chunk::Open(c)),
            ')' => Some(Chunk::Close('(')),
            ']' => Some(Chunk::Close('[')),
            '>' => Some(Chunk::Close('<')),
            '}' => Some(Chunk::Close('{')),
            _ => None,
        }
    }

    fn complete(char: Option<char>) -> Option<Self> {
        if let Some(c) = char {
            match c {
                '(' => Some(Chunk::Close(')')),
                '[' => Some(Chunk::Close(']')),
                '{' => Some(Chunk::Close('}')),
                '<' => Some(Chunk::Close('>')),
                _ => None,
            }
        } else {
            None
        }
    }
}

fn illegal_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn auto_complete_score(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

pub fn part_1(lines: Vec<String>) -> usize {
    let mut scores = vec![];
    let mut chunks = vec![];
    'outer: for line in lines {
        chunks.clear();
        let chars = line_to_chars(&line);
        for c in chars {
            if let Some(chunk) = Chunk::parse(c) {
                match chunk {
                    Chunk::Open(_) => chunks.push(c),
                    Chunk::Close(cc) => {
                        if let Some(oc) = chunks.pop() {
                            if oc != cc {
                                scores.push(illegal_score(c));
                                continue 'outer;
                            }
                        }
                    }
                }
            }
        }
    }

    scores.iter().sum::<usize>()
}

pub fn part_2(lines: Vec<String>) -> usize {
    let mut scores = vec![];
    let mut chunks = vec![];
    'outer: for line in lines {
        chunks.clear();
        let chars = line_to_chars(&line);
        for c in chars {
            if let Some(chunk) = Chunk::parse(c) {
                match chunk {
                    Chunk::Open(_) => chunks.push(c),
                    Chunk::Close(cc) => {
                        if let Some(oc) = chunks.pop() {
                            if oc != cc {
                                continue 'outer;
                            }
                        }
                    }
                }
            }
        }
        let score = (0..chunks.len()).fold(0, |mut acc: usize, _| {
            if let Some(chunk) = Chunk::complete(chunks.pop()) {
                match chunk {
                    Chunk::Close(cc) => {
                        acc = acc * 5 + auto_complete_score(cc);
                        acc
                    }
                    _ => acc,
                }
            } else {
                acc
            }
        });
        scores.push(score);
    }
    scores.sort();
    *scores.get(scores.len() / 2).unwrap()
}
