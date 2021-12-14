use std::collections::HashMap;

struct LowPoint {
    x: usize,
    y: usize,
    value: usize,
}

impl LowPoint {
    fn new(x: usize, y: usize, value: usize) -> Self {
        Self { x, y, value }
    }
}

fn parse_line(line: &String) -> Vec<u8> {
    line.split("")
        .filter(|v| !v.is_empty())
        .map(|x| x.parse::<u8>().unwrap())
        .collect()
}

fn line_to_map<'a>(
    heightmap: &'a mut HashMap<(usize, usize), u8>,
    points: Vec<u8>,
    y: usize,
) -> &'a mut HashMap<(usize, usize), u8> {
    points.iter().enumerate().for_each(|(x, p)| {
        heightmap.insert((x, y), *p);
    });
    heightmap
}

fn get_low_points(lines: Vec<String>) -> (Vec<LowPoint>, HashMap<(usize, usize), u8>) {
    let mut heightmap: HashMap<(usize, usize), u8> = HashMap::new();
    let mut low_points: Vec<LowPoint> = vec![];

    let total_line = lines.len();
    let mut point_lines = lines.iter().peekable();
    for y in 0..total_line {
        let current_points = parse_line(point_lines.next().unwrap());
        if y == 0 {
            line_to_map(&mut heightmap, current_points.clone(), y);
        }
        if let Some(next_line) = point_lines.peek() {
            line_to_map(&mut heightmap, parse_line(&next_line), y + 1);
        };

        current_points
            .iter()
            .peekable()
            .enumerate()
            .for_each(|(x, p)| {
                let up = if y > 0 {
                    *heightmap.get(&(x, y - 1)).unwrap()
                } else {
                    10
                };
                let down = if y < total_line - 1 {
                    *heightmap.get(&(x, y + 1)).unwrap()
                } else {
                    10
                };
                let left = if x > 0 {
                    *heightmap.get(&(x - 1, y)).unwrap()
                } else {
                    10
                };
                let right = if x < current_points.len() - 1 {
                    *heightmap.get(&(x + 1, y)).unwrap()
                } else {
                    10
                };
                if [up, down, left, right].iter().min().unwrap() > p {
                    low_points.push(LowPoint::new(x, y, *p as usize));
                }
            });
    }

    (low_points, heightmap)
}
pub fn part_1(lines: Vec<String>) -> usize {
    let low_points = get_low_points(lines).0;
    low_points.iter().map(|lp| lp.value).sum::<usize>() + low_points.len() as usize
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

fn update_basin<'a>(
    basins: &'a mut HashMap<(usize, usize), u8>,
    (x, y): (usize, usize),
    avoid: Direction,
    y_limit: usize,
    x_limit: usize,
    mut heightmap: &'a mut HashMap<(usize, usize), u8>,
) -> &'a mut HashMap<(usize, usize), u8> {
    let p = *heightmap.get(&(x, y)).unwrap();
    if p != 9 && basins.get(&(x, y)) == None {
        basins.insert((x, y), p);
        if avoid != Direction::Up && y > 0 {
            update_basin(
                basins,
                (x, y - 1),
                Direction::Down,
                y_limit,
                x_limit,
                &mut heightmap,
            );
        }

        if avoid != Direction::Down && y < y_limit {
            update_basin(
                basins,
                (x, y + 1),
                Direction::Up,
                y_limit,
                x_limit,
                &mut heightmap,
            );
        }

        if avoid != Direction::Left && x > 0 {
            update_basin(
                basins,
                (x - 1, y),
                Direction::Right,
                y_limit,
                x_limit,
                &mut heightmap,
            );
        }

        if avoid != Direction::Right && x < x_limit {
            update_basin(
                basins,
                (x + 1, y),
                Direction::Left,
                y_limit,
                x_limit,
                &mut heightmap,
            );
        }
    }
    basins
}

pub fn part_2(lines: Vec<String>) -> usize {
    let total_line = lines.len();
    let line_count = parse_line(&lines.clone().pop().unwrap()).len();
    let (low_points, mut heightmap) = get_low_points(lines);

    let mut basins = vec![];
    for LowPoint { x, y, value: _ } in low_points {
        let mut basin: HashMap<(usize, usize), u8> = HashMap::new();

        // Start up
        update_basin(
            &mut basin,
            (x, y),
            Direction::None,
            total_line - 1,
            line_count - 1,
            &mut heightmap,
        );

        basins.push(basin.len());
    }

    basins.sort();
    let total_basin = basins.len();
    basins
        .get(total_basin - 3..total_basin)
        .unwrap()
        .iter()
        .product::<usize>()
}
