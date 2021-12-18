use std::collections::HashMap;

use crate::days::utils::line_to_int;
struct EnergyLevel {
    value: u8,
    deps: Vec<u8>,
}

impl EnergyLevel {
    fn new(value: u8) -> Self {
        Self {
            value,
            deps: vec![],
        }
    }

    fn add_deps(&mut self, deps: Vec<u8>) {
        self.deps = deps;
    }

    fn flashes(&self, steps: u8) -> usize {
        let mut flashes = 0;
        for s in 1..=steps {
            if (self.value + s) % 10 == 0 {
                flashes += 1;
                continue;
            }
        }
        flashes
    }
}

fn line_to_map<'a>(
    energy_levels_map: &'a mut HashMap<(usize, usize), EnergyLevel>,
    energy_levels: Vec<u8>,
    y: usize,
) -> &'a mut HashMap<(usize, usize), EnergyLevel> {
    energy_levels.iter().enumerate().for_each(|(x, p)| {
        energy_levels_map.insert((x, y), EnergyLevel::new(*p));
    });
    energy_levels_map
}

fn create_map(lines: Vec<String>) -> Vec<EnergyLevel> {
    let mut energy_levels_map: HashMap<(usize, usize), EnergyLevel> = HashMap::new();
    let mut vertical_lines = lines.iter().peekable();

    // line_to_map(&mut energy_levels_map, line_to_int(vertical_lines.peek().unwrap()), 0);
    //   line_to_int(vertical_lines.peek().unwrap()).iter().enumerate().for_each(|(x, p)| {
    //     energy_levels_map.insert((x, 0), EnergyLevel::new(*p, vec![]));
    // });
    for y in 0..10 {
        let current_line = line_to_int(vertical_lines.next().unwrap());
        if y == 0 {
            line_to_map(&mut energy_levels_map, current_line.clone(), y);
        }
        if let Some(next_line) = vertical_lines.peek() {
            line_to_map(&mut energy_levels_map, line_to_int(&next_line), y + 1);
        };
        current_line.iter().enumerate().for_each(|(x, p)| {
            let mut deps = vec![];
            [
                (x, y - 1),
                (x, y + 1),
                (x - 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
            ]
            .iter()
            .for_each(|(x, y)| {
                if let Some(l) = energy_levels_map.get(&(*x, *y)) {
                    deps.push(l.value);
                }
            });
            energy_levels_map
                .entry((x, y))
                .and_modify(|l| l.add_deps(deps));
        });
    }
    todo!()
}
