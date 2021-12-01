struct Measurement<T> {
    previous: T,
    total_inc: i32,
}

impl<T> Measurement<T> {
    fn new(previous: T, total: i32) -> Measurement<T> {
        Measurement {
            previous,
            total_inc: total,
        }
    }

    fn update(&mut self, previous: T, total: i32) {
        self.previous = previous;
        self.total_inc = total;
    }
}
pub fn part_1(lines: Vec<String>) -> i32 {
    let intital = Measurement::new(lines.clone().pop().unwrap().parse::<i32>().unwrap(), 0);
    return lines
        .iter()
        .fold(intital, |mut acc, m| {
            let mut total = acc.total_inc;
            let current = m.parse::<i32>().unwrap();
            if current > acc.previous {
                total = total + 1;
            }
            acc.update(current, total);
            acc
        })
        .total_inc;
}

pub fn part_2(lines: Vec<String>) -> i32 {
    let intital = Measurement::new(Vec::new(), 0);
    return lines
        .iter()
        .fold(intital, |mut acc, m| {
            let mut previous = acc.previous;
            let mut total_inc = acc.total_inc;
            previous.push(m.parse::<i32>().unwrap());
            if previous.len() == 4 {
                let sliding_b = previous.get(1..4).unwrap().iter();
                let sliding_a = previous.get(0..3).unwrap().iter();
                if sliding_b.sum::<i32>() > sliding_a.sum::<i32>() {
                    total_inc = acc.total_inc + 1;
                }
                previous.remove(0);
            }
            acc.previous = previous;
            acc.total_inc = total_inc;
            acc
        })
        .total_inc;
}
