mod days;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    // Day 1
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

    // Day 2
    println!(
        "Day 2 Part 1 test : {}",
        days::two::part_1(read_input("2test"))
    ); 
    
    println!(
        "Day 2 Part 1 : {}",
        days::two::part_1(read_input("2"))
    );

    println!(
        "Day 2 Part 2 test : {}",
        days::two::part_2(read_input("2test"))
    ); 

    println!(
        "Day 2 Part 2 : {}",
        days::two::part_2(read_input("2"))
    );
}

// Returns an Iterator to the Reader of the lines of the file.
fn read_input(day: &str) -> Vec<String> {
    let file = File::open(format!("input/day{}.txt", day)).expect("Error opening file");
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
}
