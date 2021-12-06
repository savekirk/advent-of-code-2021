use std::collections::HashMap;

struct ReportCount {
    cols: Vec<(i32, i32)>,
}

impl ReportCount {
    fn new() -> Self {
        ReportCount { cols: Vec::new() }
    }

    fn update(&mut self, numbers: Vec<(i32, i32)>) {
        numbers.iter().enumerate().for_each(|(i, col)| {
            let existing = self.cols.get(i).unwrap_or(&(0, 0));
            let updated = (existing.0 + col.0, existing.1 + col.1);
            if self.cols.len() > i {
                self.cols[i] = updated;
            } else {
                self.cols.push(updated);
            }
        });
    }

    fn gamma_rate(&self) -> String {
        self.cols.iter().fold(String::new(), |mut binary, col| {
            binary.push(most(col));
            binary
        })
    }

    fn epsilon_rate(&self) -> String {
        self.cols.iter().fold(String::new(), |mut binary, col| {
            binary.push(least(col));
            binary
        })
    }
}

fn least(bits: &(i32, i32)) -> char {
    if bits.0 > bits.1 {
        '1'
    } else if bits.0 < bits.1 {
        '0'
    } else {
        '0'
    }
}

fn most(bits: &(i32, i32)) -> char {
    if bits.1 > bits.0 {
        '1'
    } else if bits.1 < bits.0 {
        '0'
    } else {
        '1'
    }
}

fn to_decimal(binary: String) -> isize {
    isize::from_str_radix(binary.as_str(), 2).unwrap()
}

fn decode_number(numbers: &str) -> Vec<(i32, i32)> {
    let mut bits = Vec::new();
    let n: Vec<_> = numbers.trim().split("").collect();
    n.iter().for_each(|b| {
        if *b == "0" {
            bits.push((1, 0))
        } else if *b == "1" {
            bits.push((0, 1))
        }
    });
    bits
}

pub fn part_1(lines: Vec<String>) -> isize {
    let report = lines.iter().fold(ReportCount::new(), |mut report, n| {
        let numbers = decode_number(n);
        report.update(numbers);
        report
    });

    to_decimal(report.gamma_rate()) * to_decimal(report.epsilon_rate())
}

pub fn part_2(lines: Vec<String>) -> isize {
    let mut reports = HashMap::new();

    for line in lines {
        let mut previous = String::new();
        let n: Vec<_> = line.trim().split("").collect();
        n.iter().skip_while(|b| b.is_empty()).for_each(|b| {
            let bit_pair = if *b == "0" { (1, 0) } else { (0, 1) };
            let count = reports
                .entry(previous.clone())
                .or_insert((0, 0, "".to_string()));
            *count = (count.0 + bit_pair.0, count.1 + bit_pair.1, line.clone());
            previous.push_str(b);
        });
    }

    let mut oxygen_rating = "".to_string();

    while let Some(bits) = reports.get(&oxygen_rating) {
        if (bits.0 + bits.1) == 1 {
            break;
        }
        oxygen_rating.push(most(&(bits.0, bits.1)))
    }

    let mut co2_rating = "".to_string();

    while let Some(bits) = reports.get(&co2_rating) {
        if (bits.0 + bits.1) == 1 {
            co2_rating = bits.2.clone();
            break;
        }
        co2_rating.push(least(&(bits.0, bits.1)))
    }

    to_decimal(oxygen_rating) * to_decimal(co2_rating)
}
