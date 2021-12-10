fn positions(lines: Vec<String>) -> Vec<i64> {
    lines
        .first()
        .unwrap()
        .trim()
        .split(',')
        .map(|l| l.parse::<i64>().unwrap())
        .fold(vec![], |mut acc, p| {
            acc.push(p);
            acc
        })
}

fn min_fuel(positions: Vec<i64>, calc: fn(i64) -> i64) -> i64 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..=max).fold(0, |min_fuel, pos| {
        let total_fuel = positions.iter().fold(0, |mut acc, p| {
            acc += calc((p - pos).abs());
            acc
        });
        if min_fuel == 0 {
            total_fuel
        } else {
            min_fuel.min(total_fuel)
        }
    })
}
pub fn part_1(lines: Vec<String>) -> i64 {
    let positions = positions(lines);
    min_fuel(positions, |diff| diff)
}

pub fn part_2(lines: Vec<String>) -> i64 {
    let positions = positions(lines);
    min_fuel(positions, |diff| diff * (diff + 1) / 2)
}
