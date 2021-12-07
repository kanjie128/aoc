use std::vec;

#[derive(Debug, Clone)]
struct Cell {
    data: i32,
    marked: bool,
}

#[derive(Debug, Clone)]
struct Board {
    board: Vec<Cell>,
    row: usize,
    col: usize,
    win: bool,
}
impl Board {
    fn new() -> Self {
        Self {
            board: vec![],
            row: 0,
            col: 0,
            win: false,
        }
    }
}

#[derive(Debug)]
struct Boards {
    boards: Vec<Board>,
    win: usize,
}

impl Boards {
    fn new() -> Self {
        Self {
            boards: vec![],
            win: 0,
        }
    }
}

fn main() {
    let s = std::fs::read_to_string("input").unwrap();

    let mut line_iter = s.lines().into_iter();

    // parse number line
    let numbers_line = line_iter.next().unwrap();
    let mut numbers = vec![];
    for n in numbers_line.split(',') {
        numbers.push(n.parse::<i32>().unwrap());
    }

    // consume newline
    let _ = line_iter.next().unwrap();

    // parse board
    let mut boards = Boards::new();
    let mut board = Board::new();
    loop {
        if let Some(line) = line_iter.next() {
            // board split by newline
            if line == "" {
                if board.board.len() > 0 {
                    board.row = board.board.len() / board.col;
                    boards.boards.push(board.clone());
                    board = Board::new();
                }
                continue;
            }

            let mut col = 0;
            for n in line.split(|c: char| c.is_ascii_whitespace()) {
                if n == "" {
                    continue;
                }
                col += 1;
                board.board.push(Cell {
                    data: n.parse::<i32>().unwrap(),
                    marked: false,
                });
            }
            board.col = col;
        } else {
            if board.board.len() > 0 {
                board.row = board.board.len() / board.col;
                boards.boards.push(board.clone());
            }
            break;
        }
    }
    let mut first_win = false;
    let board_count = boards.boards.len();
    for n in numbers {
        // mark board
        for board in boards.boards.iter_mut() {
            if board.win {
                continue;
            }
            for cell in board.board.iter_mut() {
                if cell.data == n {
                    cell.marked = true;
                }
            }
            // board win
            if let Some(m) = board_win(board) {
                board.win = true;
                //first win
                if !first_win {
                    first_win = true;
                    println!("first win unmarked sum: {}, n: {}, multi: {}", m, n, n * m);
                }
                //last win
                if boards.win == board_count - 1 {
                    println!("last win unmarked sum: {}, n: {}, multi: {}", m, n, n * m);
                    return;
                } else {
                    boards.win += 1;
                }
            }
        }
    }
}

fn board_win(board: &Board) -> Option<i32> {
    for i in 0..board.row {
        let mut row_win = true;
        let mut col_win = true;
        for j in 0..board.col {
            //row
            let cell = board.board.get(i * board.col + j).unwrap();
            if !cell.marked {
                row_win = false;
            }
            //col
            let cell = board.board.get(j * board.col + i).unwrap();
            if !cell.marked {
                col_win = false;
            }
            if !row_win && !col_win {
                break;
            }
        }
        if row_win || col_win {
            let mut sum = 0;
            for cell in &board.board {
                if !cell.marked {
                    sum += cell.data;
                }
            }
            return Some(sum);
        }
    }
    None
}

#[allow(dead_code)]
fn dump_board(board: &Board) {
    for i in 0..board.row {
        for j in 0..board.col {
            let cell = board.board.get(i * board.col + j).unwrap();
            if cell.marked {
                print!("[{}] ", cell.data)
            } else {
                print!("{} ", cell.data)
            }
        }
        println!();
    }
    println!();
}
