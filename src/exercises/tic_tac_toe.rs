use std::io;

const BOARD_SIZE: usize = 3;
const X_SYMBOL: char = 'X';
const O_SYMBOL: char = 'O';
const EMPTY_SYMBOL: char = '-';

#[derive(Debug, Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    Taken(Player),
}

struct Board {
    cells: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn display(&self) {
        for row in &self.cells {
            for cell in row {
                let symbol = match cell {
                    Cell::Empty => EMPTY_SYMBOL,
                    Cell::Taken(Player::O) => O_SYMBOL,
                    Cell::Taken(Player::X) => X_SYMBOL,
                };
                print!(" {symbol} ");
            }
            println!();
        }
    }

    fn make_move(&mut self, col: usize, row: usize, player: Player) {
        self.cells[row][col] = Cell::Taken(player);
    }

    fn is_valid_move(&self, col: usize, row: usize) -> bool {
        row < BOARD_SIZE && col < BOARD_SIZE && self.cells[row][col] == Cell::Empty
    }

    fn is_board_full(&self) -> bool {
        for row in &self.cells {
            for cell in row {
                if *cell == Cell::Empty {
                    return false;
                }
            }
        }
        true
    }

    fn is_winner(&self, player: Player) -> bool {
        // Check rows, columns, and diagonals
        for i in 0..BOARD_SIZE {
            if (self.cells[i][0] == Cell::Taken(player)
                && self.cells[i][1] == Cell::Taken(player)
                && self.cells[i][2] == Cell::Taken(player))
                || (self.cells[0][i] == Cell::Taken(player)
                    && self.cells[1][i] == Cell::Taken(player)
                    && self.cells[2][i] == Cell::Taken(player))
            {
                return true;
            }
        }
        if (self.cells[0][0] == Cell::Taken(player)
            && self.cells[1][1] == Cell::Taken(player)
            && self.cells[2][2] == Cell::Taken(player))
            || (self.cells[0][2] == Cell::Taken(player)
                && self.cells[1][1] == Cell::Taken(player)
                && self.cells[2][0] == Cell::Taken(player))
        {
            return true;
        }
        false
    }
}

pub fn run() {
    let mut board = Board::new();
    let mut current_player = Player::X;

    loop {
        board.display();
        println!(
            "Player {:?} enter move. Enter col(0-2) row(0-2)",
            current_player
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let coordinates: Vec<usize> = input
            .trim()
            .split_whitespace()
            .filter_map(|value| value.parse().ok())
            .collect();
        if coordinates.len() != 2 {
            println!("Invalid coordinates");
            continue;
        }
        let col = coordinates[0];
        let row = coordinates[1];

        if !board.is_valid_move(col, row) {
            println!("Invalid move");
            continue;
        }

        board.make_move(col, row, current_player);

        if board.is_winner(current_player) {
            println!("Player {:?} wins", current_player);
            break;
        }

        if board.is_board_full() {
            print!("The game has ended");
            break;
        }

        current_player = if current_player == Player::X {
            Player::O
        } else {
            Player::X
        }
    }
}
