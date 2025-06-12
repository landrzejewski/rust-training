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
        write!(f, "{}", symbol)
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
            Cell::Taken(player) => write!(f, "{}", player),
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

    /*fn display(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                print!("{}", cell);
            }
            println!();
        }
    }*/

    /*fn display(&self) {
        self.cells.iter().for_each(|row| {
            row.iter().for_each(|cell| print!("{}", cell));
            println!();
        });
    }*/

    fn display(&self) {
        self.board
            .iter()
            .map(|row| row.iter().map(|cell| cell.to_string()).collect::<String>())
            .for_each(|row_str| println!("{}", row_str));
    }

    fn make_move(&mut self, col: usize, row: usize, player: Player) -> bool {
        if (self.is_valid_move(col, row)) {
            self.board[row][col] = Cell::Taken(player);
            true
        } else {
            false
        }
    }

    fn is_valid_move(&self, col: usize, row: usize) -> bool {
        row < BOARD_SIZE && col < BOARD_SIZE && self.board[row][col] == Cell::Empty
    }

    /*fn is_board_full(&self) -> bool {
        for row in &self.board {
            for cell in row {
                if *cell == Cell::Empty {
                    return false;
                }
            }
        }
        true
    }*/

    fn is_boards_full(&self) -> bool {
        self.board.iter().flatten().all(|&cell| cell != Cell::Empty)
    }

    /*fn is_winner(&self, player: Player) -> bool {
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
    }*/

    fn is_winner(&self, player: Player) -> bool {
        // Check rows
        if self
            .board
            .iter()
            .any(|row| row.iter().all(|&cell| cell == Cell::Taken(player)))
        {
            return true;
        }
        // Check columns
        if (0..BOARD_SIZE)
            .any(|col| (0..BOARD_SIZE).all(|row| self.board[row][col] == Cell::Taken(player)))
        {
            return true;
        }
        // Check main diagonal
        if (0..BOARD_SIZE).all(|i| self.board[i][i] == Cell::Taken(player)) {
            return true;
        }
        // Check anti-diagonal
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

        if game.is_boards_full() {
            print!("The game has ended");
            break;
        }

        game.toggle_player();
    }
}
