pub fn part_1(lines: Vec<String>) -> usize {
    let mut days_left: [usize; 9] = [0; 9];
    lines
        .first()
        .unwrap()
        .trim()
        .split(',')
        .map(|l| l.parse::<usize>().unwrap())
        .for_each(|dl| days_left[dl] += 1);
    generate(&mut days_left, 80)
}

fn generate(days_left: &mut [usize; 9], total_days: u16) -> usize {
    for _ in 1..=total_days {
        days_left.rotate_left(1);
        days_left[6] += days_left[8];
    }

    days_left.iter().sum()
}

pub fn part_2(lines: Vec<String>) -> usize {
  let mut days_left: [usize; 9] = [0; 9];
  lines
      .first()
      .unwrap()
      .trim()
      .split(',')
      .map(|l| l.parse::<usize>().unwrap())
      .for_each(|dl| days_left[dl] += 1);
  generate(&mut days_left, 256)
}
