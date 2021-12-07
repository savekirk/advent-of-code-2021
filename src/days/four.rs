use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Number {
    Marked,
    Unmarked,
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    row: i32,
    col: i32,
    state: Number,
}

impl Cell {
    fn new(row: i32, col: i32, state: Number) -> Self {
        Self { row, col, state }
    }

    fn mark(&mut self) {
        self.state = Number::Marked
    }
}

#[derive(Debug, Clone)]
struct Board {
    cells: HashMap<i32, Cell>,
    unmarked_sum: i32,
    has_won: bool,
    marked_rows: HashMap<i32, Vec<i32>>,
    marked_cols: HashMap<i32, Vec<i32>>,
}

impl Board {
    fn new() -> Self {
        Board {
            unmarked_sum: 0,
            has_won: false,
            cells: HashMap::new(),
            marked_rows: HashMap::new(),
            marked_cols: HashMap::new(),
        }
    }

    fn new_cell(&mut self, number: i32, row: i32, col: i32, state: Number) {
        let cell = Cell::new(row, col, state);
        self.cells.insert(number, cell);
        self.un_mark(number);
        if let Number::Marked = state {
            self.mark(&cell, number);
        }
    }

    fn mark(&mut self, cell: &Cell, number: i32) {
        let marked_rows = self
            .marked_rows
            .entry(cell.row)
            .and_modify(|numbers| numbers.push(number))
            .or_insert(vec![number]);

        let marked_cols = self
            .marked_cols
            .entry(cell.col)
            .and_modify(|numbers| numbers.push(number))
            .or_insert(vec![number]);
        self.unmarked_sum -= number;
        if marked_cols.len() == 5 || marked_rows.len() == 5 {
            self.has_won = true;
        }
    }

    fn change_cell_state(&mut self, drawn: i32) -> Option<Cell> {
        if let Some(cell) = self.cells.get_mut(&drawn) {
            cell.mark();
            Some(cell.to_owned())
        } else {
            None
        }
    }

    fn un_mark(&mut self, number: i32) {
        self.unmarked_sum += number;
    }
}

struct Game {
    drawable_numbers: Vec<i32>,
    boards: Vec<Board>,
    drawn: Vec<i32>,
}

impl Game {
    fn new(drawable_numbers: Vec<i32>) -> Self {
        Self {
            boards: vec![],
            drawable_numbers,
            drawn: vec![],
        }
    }

    fn new_board(&mut self, board: Board) {
        self.boards.push(board);
    }

    fn draw(&mut self, count: usize) -> Vec<i32> {
        let drawn = self.drawable_numbers.drain(..count).collect::<Vec<i32>>();
        for d in &drawn {
            self.drawn.push(*d);
        }
        drawn
    }

    fn get_score(&mut self, last_to_win: bool) -> i32 {
        let mut total_board_won = 0;
        let total_board = self.boards.len();
        let mut score = 0;
        for drawn in &self.drawable_numbers {
            for board in &mut self.boards {
                if board.has_won {
                    continue;
                }
                if let Some(cell) = board.change_cell_state(*drawn) {
                    board.mark(&cell, *drawn);
                    if board.has_won {
                        total_board_won += 1;
                        score = board.unmarked_sum * drawn;
                        if last_to_win {
                            if total_board_won == total_board {
                                return score;
                            }
                        } else {
                            return score;
                        }
                    };
                }
            }
        }

        score
    }
}

fn create_game(lines: Vec<String>) -> Game {
    let drawable = lines
        .first()
        .unwrap()
        .trim()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut game = Game::new(drawable);

    let drawn = game.draw(5);

    let mut board = Board::new();
    let mut row = 0;
    for line in lines.iter().skip(2) {
        if line.trim().is_empty() {
            game.new_board(board);
            board = Board::new();
            row = 0;
            continue;
        }

        line.trim()
            .split(" ")
            .into_iter()
            .skip_while(|n| n.trim().is_empty())
            .map(|n| n.parse::<i32>().unwrap_or(-1))
            .filter(|n| *n > -1)
            .enumerate()
            .for_each(|(col, n)| {
                let state = if drawn.contains(&n) {
                    Number::Marked
                } else {
                    Number::Unmarked
                };
                board.new_cell(n, row, col as i32, state);
            });
        row += 1;
    }

    game
}

pub fn part_1(lines: Vec<String>) -> i32 {
    create_game(lines).get_score(false)
}

pub fn part_2(lines: Vec<String>) -> i32 {
    create_game(lines).get_score(true)
}