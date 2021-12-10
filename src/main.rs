mod days;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    // // Day 1
    // println!(
    //     "Day 1 Part 1 test : {}",
    //     days::one::part_1(read_input("1test"))
    // );
    // println!("Day 1 Part 1 : {}", days::one::part_1(read_input("1")));

    // println!(
    //     "Day 1 Part 2 test : {}",
    //     days::one::part_2(read_input("1test"))
    // );
    // println!("Day 1 Part 2 : {}", days::one::part_2(read_input("1")));

    // // Day 2
    // println!(
    //     "Day 2 Part 1 test : {}",
    //     days::two::part_1(read_input("2test"))
    // );

    // println!(
    //     "Day 2 Part 1 : {}",
    //     days::two::part_1(read_input("2"))
    // );

    // println!(
    //     "Day 2 Part 2 test : {}",
    //     days::two::part_2(read_input("2test"))
    // );

    // println!(
    //     "Day 2 Part 2 : {}",
    //     days::two::part_2(read_input("2"))
    // );

    // Day 3
    // println!(
    //     "Day 3 Part 1 test : {}",
    //     days::three::part_1(read_input("3test"))
    // );

    // println!(
    //     "Day 3 Part 1 : {}",
    //     days::three::part_1(read_input("3"))
    // );

    // println!(
    //     "Day 3 Part 2 test : {}",
    //     days::three::part_2(read_input("3test"))
    // );

    // println!(
    //     "Day 3 Part 2 : {}",
    //     days::three::part_2(read_input("3"))
    // );

    // println!(
    //     "Day 4 Part 1 test : {}",
    //     days::four::part_1(read_input("4test"))
    // );

    // println!("Day 4 Part 1 : {}", days::four::part_1(read_input("4")));

    // println!(
    //     "Day 4 Part 2 test : {}",
    //     days::four::part_2(read_input("4test"))
    // );

    // println!("Day 4 Part 2 : {}", days::four::part_2(read_input("4")));

    // println!(
    //     "Day 5 Part 1 test : {}",
    //     days::five::part_1(read_input("5test"))
    // );

    // println!("Day 5 Part 1: {}", days::five::part_1(read_input("5")));

    // println!(
    //     "Day 5 Part 2 test : {}",
    //     days::five::part_2(read_input("5test"))
    // );

    // println!("Day 5 Part 2: {}", days::five::part_2(read_input("5")));

    // println!(
    //     "Day 6 Part 1 test : {}",
    //     days::six::part_1(read_input("6test"))
    // );

    // println!("Day 6 Part 1 : {}", days::six::part_1(read_input("6")));

    // println!("Day 6 Part 1 : {}", days::six::part_2(read_input("6")));

    // println!(
    //     "Day 7 Part 1 test : {}",
    //     days::seven::part_1(read_input("7test"))
    // );

    // println!("Day 7 Part 1 : {}", days::seven::part_1(read_input("7")));

    println!(
        "Day 7 Part 2 test : {}",
        days::seven::part_2(read_input("7test"))
    );

    println!("Day 7 Part 2 : {}", days::seven::part_2(read_input("7")));
}

// Returns an Iterator to the Reader of the lines of the file.
fn read_input(day: &str) -> Vec<String> {
    let file = File::open(format!("input/day{}.txt", day)).expect("Error opening file");
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
}
