mod days;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    println!(
        "Day 1 Part 1 test : {}",
        days::one::part_1(read_input("1test"))
    );
    println!("Day 1 Part 1 : {}", days::one::part_1(read_input("1")));

    println!(
        "Day 1 Part 2 test : {}",
        days::one::part_2(read_input("1test"))
    );
    println!("Day 1 Part 2 : {}", days::one::part_2(read_input("1")));
}

// Returns an Iterator to the Reader of the lines of the file.
fn read_input(day: &str) -> Vec<String> {
    let file = File::open(format!("input/day{}.txt", day)).expect("Error opening file");
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
}
