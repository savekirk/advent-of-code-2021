enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
    Invalid
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32
}

impl Position {
    fn new() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0
        }
    }

    fn forward(&mut self, value: i32) {
        self.horizontal = self.horizontal + value;
    }

    fn down(&mut self, value: i32) {
        self.depth = self.depth + value;
    }

    fn up(&mut self, value: i32) {
        self.depth = self.depth - value;
    }

    fn process_command_1(&mut self, command: Command) {
        match command {
            Command::Forward(value) => self.forward(value),
            Command::Down(value) => self.down(value),
            Command::Up(value) => self.up(value),
            Command::Invalid => {},
        }
    }
    
    fn process_command_2(&mut self, command: Command) {
        match command {
            Command::Forward(value) => { self.forward(value); self.down(value * self.aim)},
            Command::Down(value) => self.aim = self.aim + value,
            Command::Up(value) => self.aim = self.aim - value,
            Command::Invalid => {},
        }
    }
}

fn parse_command(command: &str) -> Command {
    let mut components = command.trim().split(" ").into_iter();
    let action = components.next().unwrap().trim();
    let value = components.next().unwrap().trim().parse::<i32>().unwrap();
    if action == "forward" {
        Command::Forward(value)
    } else if action == "down" {
        Command::Down(value)
    } else if action == "up" {
        Command::Up(value)
    } else {
        Command::Invalid
    }
}

fn final_position(lines: Vec<String>, f: fn(Command, Position) -> Position) -> i32 {
    let position= lines
    .iter()
    .fold(Position::new(), |position, c| {
        let command = parse_command(c);
        f(command, position)
    });

    position.depth * position.horizontal
}
pub fn part_1(lines: Vec<String>) -> i32 {
    final_position(lines, |command, mut position| { position.process_command_1(command); position })
}

pub fn part_2(lines: Vec<String>) -> i32 {
    final_position(lines, |command, mut position| { position.process_command_2(command); position })
}
