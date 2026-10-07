use std::fmt;
use std::io::{self, Write};

const BOARD_SIZE: usize = 3;
const FIELD_COUNT: usize = BOARD_SIZE * BOARD_SIZE;

#[derive(Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

impl Player {
    fn toggle(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Player::X => 'X',
            Player::O => 'O',
        };
        write!(f, "{symbol}")
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Taken(Player),
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "-"),
            Cell::Taken(player) => write!(f, "{player}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum MoveError {
    OutOfBounds,
    Occupied,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveError::OutOfBounds => write!(f, "field is off the board"),
            MoveError::Occupied => write!(f, "field is already taken"),
        }
    }
}

impl std::error::Error for MoveError {}

struct Board {
    cells: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn make_move(&mut self, field: usize, player: Player) -> Result<(), MoveError> {
        if field >= FIELD_COUNT {
            return Err(MoveError::OutOfBounds);
        }
        let (row, col) = (field / BOARD_SIZE, field % BOARD_SIZE);
        if self.cells[row][col] != Cell::Empty {
            return Err(MoveError::Occupied);
        }
        self.cells[row][col] = Cell::Taken(player);
        Ok(())
    }

    fn is_full(&self) -> bool {
        self.cells
            .iter()
            .flatten()
            .all(|&cell| cell != Cell::Empty)
    }

    fn is_winner(&self, player: Player) -> bool {
        let target = Cell::Taken(player);

        let rows = self.cells.iter().any(|row| row.iter().all(|&c| c == target));

        let cols = (0..BOARD_SIZE).any(|col| self.cells.iter().all(|row| row[col] == target));

        let main_diag = (0..BOARD_SIZE).all(|i| self.cells[i][i] == target);

        let anti_diag = (0..BOARD_SIZE).all(|i| self.cells[i][BOARD_SIZE - 1 - i] == target);

        rows || cols || main_diag || anti_diag
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum InputError {
    Io(io::Error),
    NotANumber,
    OutOfRange,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::Io(err) => write!(f, "read error: {err}"),
            InputError::NotANumber => write!(f, "not a valid number"),
            InputError::OutOfRange => write!(f, "the number must be in the range 1-{FIELD_COUNT}"),
        }
    }
}

impl std::error::Error for InputError {}

impl From<io::Error> for InputError {
    fn from(err: io::Error) -> Self {
        InputError::Io(err)
    }
}

fn read_field() -> Result<usize, InputError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let number: usize = input.trim().parse().map_err(|_| InputError::NotANumber)?;

    if !(1..=FIELD_COUNT).contains(&number) {
        return Err(InputError::OutOfRange);
    }

    Ok(number - 1)
}

pub fn run() {
    let mut board = Board::new();
    let mut player = Player::X;

    loop {
        print!("{board}");
        println!("Player {player}, enter your move. Choose a field (1-{FIELD_COUNT}):");
        io::stdout().flush().ok();

        let field = match read_field() {
            Ok(field) => field,
            Err(err) => {
                println!("{err}");
                continue;
            }
        };

        if let Err(err) = board.make_move(field, player) {
            println!("Invalid move: {err}");
            continue;
        }

        if board.is_winner(player) {
            print!("{board}");
            println!("Player {player} wins!");
            break;
        }

        if board.is_full() {
            print!("{board}");
            println!("Draw - the game is over.");
            break;
        }

        player = player.toggle();
    }
}
