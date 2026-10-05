use std::io;

const BOARD_SIZE: usize = 3;
const EMPTY: char = '-';

type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

fn display(board: &Board) {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            print!("{}", board[row][col]);
        }
        println!();
    }
}

fn read_field() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim()
        .parse::<i32>()
        .unwrap_or(-1)
}

fn make_move(board: &mut Board, field: usize, player: char) -> bool {
    let row = field / BOARD_SIZE;
    let col = field % BOARD_SIZE;
    if  board[row][col] == EMPTY {
        board[row][col] = player;
        true
    } else {
        false
    }
}

fn is_board_full(board: &Board) -> bool {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if board[row][col] == EMPTY {
                return false;
            }
        }
    }
    true
}

fn is_winner(board: &Board, player: char) -> bool {
    for row in 0..BOARD_SIZE {
        let mut all = true;
        for col in 0..BOARD_SIZE {
            if board[row][col] != player {
                all = false;
            }
        }
        if all {
            return true;
        }
    }

    for col in 0..BOARD_SIZE {
        let mut all = true;
        for row in 0..BOARD_SIZE {
            if board[row][col] != player {
                all = false;
            }
        }
        if all {
            return true;
        }
    }

    let mut all = true;
    for i in 0..BOARD_SIZE {
        if board[i][i] != player {
            all = false;
        }
    }
    if all {
        return true;
    }

    let mut all = true;
    for i in 0..BOARD_SIZE {
        if board[i][BOARD_SIZE - 1 - i] != player {
            all = false;
        }
    }
    if all {
        return true;
    }

    false
}

fn toggle_player(player: char) -> char {
    if player == 'X' {
        'O'
    } else {
        'X'
    }
}

pub fn run() {
    let mut board: Board = [[EMPTY; BOARD_SIZE]; BOARD_SIZE];
    let mut player = 'X';

    loop {
        display(&board);

        println!("Player {} enter move. Enter field (1-9)", player);
        let field_number = read_field();
        if field_number == -1 || (field_number < 1 && field_number > 9) {
            println!("Enter a number from 1 to {}", BOARD_SIZE * BOARD_SIZE);
            continue;
        }

        let field = (field_number as usize) - 1;

        if !make_move(&mut board, field, player) {
            println!("Invalid move");
            continue;
        }

        if is_winner(&board, player) {
            display(&board);
            println!("Player {} wins", player);
            break;
        }

        if is_board_full(&board) {
            display(&board);
            println!("The game has ended in a draw");
            break;
        }

        player = toggle_player(player);

    }

}