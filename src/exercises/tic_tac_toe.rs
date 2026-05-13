use std::fmt::Display;
use std::io;

const BOARD_SIZE: usize = 3;

#[derive(Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Player::X => "X",
            Player::O => "O",
        };
        write!(f, "{symbol}")
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    Taken(Player),
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "-"),
            Cell::Taken(player) => write!(f, "{player}"),
        }
    }
}

struct TicTacToe {
    board: [[Cell; BOARD_SIZE]; BOARD_SIZE],
    player: Player,
}

impl TicTacToe {
    fn new() -> TicTacToe {
        TicTacToe {
            board: [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
            player: Player::X,
        }
    }

    fn toggle_player(&mut self) {
        match self.player {
            Player::X => self.player = Player::O,
            Player::O => self.player = Player::X,
        }
    }

    fn display(&self) {
        for row in self.board.iter() {
            for cell in row.iter() {
                print!("{cell}");
            }
            println!();
        }
    }

    fn make_move(&mut self, col: usize, row: usize, player: Player) -> bool {
        if self.is_valid_move(col, row) {
            self.board[row][col] = Cell::Taken(player);
            true
        } else {
            false
        }
    }

    fn is_valid_move(&self, col: usize, row: usize) -> bool {
        row < BOARD_SIZE && col < BOARD_SIZE && self.board[row][col] == Cell::Empty
    }

    fn is_board_full(&self) -> bool {
        for row in &self.board {
            for cell in row {
                if *cell == Cell::Empty {
                    return false;
                }
            }
        }
        true
    }

    fn is_winner(&self, player: Player) -> bool {
        if self
            .board
            .iter()
            .any(|row| row.iter().all(|&cell| cell == Cell::Taken(player)))
        {
            return true;
        }

        if (0..BOARD_SIZE)
            .any(|col| (0..BOARD_SIZE).all(|row| self.board[row][col] == Cell::Taken(player)))
        {
            return true;
        }

        if (0..BOARD_SIZE).all(|i| self.board[i][i] == Cell::Taken(player)) {
            return true;
        }

        if (0..BOARD_SIZE).all(|i| self.board[i][BOARD_SIZE - 1 - i] == Cell::Taken(player)) {
            return true;
        }
        false
    }
}

fn read_coordinates() -> Option<(usize, usize)> {
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
        return None;
    }
    let col = coordinates[0] - 1;
    let row = coordinates[1] - 1;
    Some((col, row))
}

pub fn run() {
    let mut game = TicTacToe::new();

    loop {
        game.display();
        println!("Player {} enter move. Enter col(1-3) row(1-3)", game.player);

        let Some((col, row)) = read_coordinates() else {
            println!("Invalid coordinates");
            continue;
        };

        if !game.make_move(col, row, game.player) {
            println!("Invalid move");
            continue;
        }

        if game.is_winner(game.player) {
            println!("Player {} wins", game.player);
            break;
        }

        if game.is_board_full() {
            print!("The game has ended");
            break;
        }

        game.toggle_player();
    }
}
