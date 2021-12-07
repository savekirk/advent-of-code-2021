use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim().split(',').collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    fn should_consider(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    fn horizontal_sort_and_diff(&self) -> (Point, Point, i32) {
        let diff = (self.start.x - self.end.x).abs();
        if self.start.x > self.end.x {
            (self.end.clone(), self.start.clone(), diff)
        } else {
            (self.start.clone(), self.end.clone(), diff)
        }
    }

    fn vertical_sort_and_diff(&self) -> (Point, Point, i32) {
        let diff = (self.start.y - self.end.y).abs();
        if self.start.y > self.end.y {
            (self.end.clone(), self.start.clone(), diff)
        } else {
            (self.start.clone(), self.end.clone(), diff)
        }
    }

    fn points(&self) -> HashSet<Point> {
        let mut points = HashSet::from_iter(vec![self.start, self.end]);
        let is_diagonal = |a: &Point, b: &Point| (a.x - b.x).abs() == (a.y - b.y).abs();

        let (mut x1, x2, diff) = self.horizontal_sort_and_diff();

        while diff > 1 && x2.x - x1.x > 1 {
            if self.should_consider() {
                x1 = Point::new(x1.x + 1, x1.y);
            } else {
                if x1.x == x1.y || (x2.x > x1.x && x2.y > x1.y) {
                    x1 = Point::new(x1.x + 1, x1.y + 1);
                } else {
                    x1 = Point::new(x1.x + 1, (x1.y - 1).abs());
                }
            }
            if self.should_consider() || (is_diagonal(&x1, &x2)) {
                points.insert(x1);
            }
        }

        let (mut y1, y2, diff) = self.vertical_sort_and_diff();

        while diff > 1 && y2.y - y1.y > 1 {
            if self.should_consider() {
                y1 = Point::new(y1.x, y1.y + 1);
            } else {
                if y1.x == y1.y || (y2.x > y2.x && y2.y > y1.y) {
                    y1 = Point::new(y1.x + 1, y1.y + 1);
                } else {
                    y1 = Point::new((y1.x - 1).abs(), y1.y + 1);
                }
            }
            if self.should_consider() || (is_diagonal(&y1, &y2)) {
                points.insert(y1);
            }
        }

        points
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line_coords: Vec<&str> = s.trim().split("->").collect();

        let start = Point::from_str(line_coords[0])?;
        let end = Point::from_str(line_coords[1])?;

        Ok(Line::new(start, end))
    }
}

pub fn get_total_overlap(lines: Vec<String>, consider_all_lines: bool) -> i32 {
    let mut diagram: HashMap<Point, i32> = HashMap::new();
    let mut total_overlap = 0;
    for line in lines {
        if let Ok(line) = Line::from_str(line.as_str()) {
            if consider_all_lines || line.should_consider() {
                for point in line.points() {
                    diagram
                        .entry(point.clone())
                        .and_modify(|p| {
                            if *p == 1 {
                                total_overlap += 1;
                            }
                            *p += 1
                        })
                        .or_insert(1);
                }
            }
        } else {
            println!("error => {:?}", line);
        }
    }
    total_overlap
}

pub fn part_1(lines: Vec<String>) -> i32 {
    get_total_overlap(lines, false)
}

pub fn part_2(lines: Vec<String>) -> i32 {
    get_total_overlap(lines, true)
}
