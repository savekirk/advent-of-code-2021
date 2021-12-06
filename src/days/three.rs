use std::default;

enum Bit {
  Zero(i32),
  One(i32)
}

impl Default for Bit {
  fn default() -> Self { Bit::Zero(0) }
}

#[derive(Default)]
struct Report {
  cols: Vec<(i32, i32)>
}

impl Report {
    fn new() -> Report {
      Report { 
        cols: Vec::new(), 
      }
    }

    fn update(&mut self, numbers: Vec<(i32, i32)>) {
      // for (i, col) in self.cols.iter().enumerate() {
      //   let number = numbers.get(i).unwrap();
      //   self.cols[i] = (number.0 + col.0, number.1 + col.1);
      // }
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
      self.cols.iter().fold(String::new(), |mut binary, col| { binary.push(most(col)); binary })
    }
    
    fn epsilon_rate(&self) -> String {
      self.cols.iter().fold(String::new(), |mut binary, col| { binary.push(least(col)); binary })
    }


}

fn least(bits: &(i32, i32)) -> char {
  if bits.0 > bits.1 {
    '1'
  } else {
    '0'
  }
}

fn most(bits: &(i32, i32)) -> char {
  if bits.1 > bits.0 {
    '1'
  } else {
    '0'
  }
}

fn to_decimal(binary: String) -> isize {
  isize::from_str_radix(binary.as_str(), 2).unwrap()
}

fn decode_number(numbers: &str) -> Vec<(i32, i32)> {
  let mut bits = Vec::new();
  let n: Vec<_> = numbers.trim().split("").collect();
  n.iter().for_each(|b| {
     let bit = b.parse::<i32>().unwrap_or(-1);
     if bit == 0 {
       bits.push((1, 0))
     } else if bit == 1 {
      bits.push((0, 1))
     }
  });
  bits
}

pub fn part_1(lines: Vec<String>) -> isize {
  let report = lines.iter().fold(Report::new(), |mut report, n| {
    let numbers = decode_number(n);
    report.update(numbers);
    report
  });
  
  to_decimal(report.gamma_rate()) * to_decimal(report.epsilon_rate())
  
}

pub fn part_2(lines: Vec<String>) -> isize {
    let mut ones: Vec<&String> = Vec::new();
    let mut zeros: Vec<&String> = Vec::new();
    let mut binary_len = 0;

    let report = lines.iter().fold(Report::new(), |mut report, n| {
      if n.starts_with("0") {
        zeros.push(n);
      } else {
        ones.push(n);
      }
      
      let numbers = decode_number(n);
      report.update(numbers);
      report
    });

    let binary_len = lines.first().unwrap().len();
    for col in report.cols {
        
    }

  78
}