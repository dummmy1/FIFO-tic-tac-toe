use std::fmt;
use std::{collections::VecDeque, io};

struct GameState {
    turn: u8,
    board: [Option<Player>; 16],
    x_player: VecDeque<u8>,
    o_player: VecDeque<u8>,
}

#[derive(Copy, Clone)]
enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

fn initialize_new_game() -> GameState {
    GameState {
        turn: 0,
        board: [None; 16],
        x_player: VecDeque::new(),
        o_player: VecDeque::new(),
    }
}

fn main() {
    loop {
        println!("Starting a new game!");
        let mut game: GameState = initialize_new_game();
        loop {
            display_board(&game);
            let index: u8 = board_index_from_input(&mut game);
            place_a_piece_on_the_board(&index, &mut game);

            if game.x_player.len() == 4 {
                if let Some(winner) = check_for_win(&game) {
                    display_board(&game);
                    println!("{} won!", winner);
                    break;
                }
            }

            game.turn = (game.turn + 1) % 2
        }
    }
}

fn check_for_win(game: &GameState) -> Option<Player> {
    let winning_indexes: [[u8; 4]; 10] = [
        // rows
        [0, 1, 2, 3],
        [4, 5, 6, 7],
        [8, 9, 10, 11],
        [12, 13, 14, 15],
        // columns
        [0, 4, 8, 12],
        [1, 5, 9, 13],
        [2, 6, 10, 14],
        [3, 7, 11, 15],
        // diagonals
        [0, 5, 10, 15],
        [3, 6, 9, 12],
        //ty ai
    ];

    let current_player = if game.turn == 0 { Player::X } else { Player::O };

    let has_won = winning_indexes.iter().any(|line| {
        line.iter()
            .all(|&i| match (&game.board[i as usize], current_player) {
                (Some(Player::X), Player::X) => true,
                (Some(Player::O), Player::O) => true,
                _ => false,
            })
    });

    if has_won {
        Some(current_player)
    } else {
        None
    }
}

fn place_a_piece_on_the_board(index: &u8, game: &mut GameState) {
    if game.turn == 0 {
        game.board[*index as usize] = Some(Player::X);
        game.x_player.push_back(*index);
        if game.x_player.len() == 5 {
            match game.x_player.pop_front() {
                Some(num) => game.board[num as usize] = None,
                None => {}
            }
        }
    } else {
        game.board[*index as usize] = Some(Player::O);
        game.o_player.push_back(*index);
        if game.o_player.len() == 5 {
            match game.o_player.pop_front() {
                Some(num) => game.board[num as usize] = None,
                None => {}
            }
        }
    }
}

fn board_index_from_input(game: &mut GameState) -> u8 {
    loop {
        if game.turn == 0 {
            println!("Where to place X? 1-16");
        } else {
            println!("Where to place O? 1-16");
        }
        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
            Err(_) => continue,
            Ok(_) => {}
        }

        match input.trim().parse::<u8>() {
            Ok(num) if num >= 1 && num <= 16 && game.board[(num - 1) as usize].is_none() => {
                return num - 1;
            }
            Ok(num) if num >= 1 && num <= 16 => println!("Spot already taken"),
            Ok(_) => println!("Must be a number from 1-16"),
            Err(_) => println!("Must be a number from 1-16"),
        }
    }
}

fn display_board(game: &GameState) {
    let expiring_piece: Option<u8> = if game.turn == 0 && game.x_player.len() == 4 {
        game.x_player.front().copied()
    } else if game.turn == 1 && game.o_player.len() == 4 {
        game.o_player.front().copied()
    } else {
        None
    };

    println!();
    println!(
        " {} | {} | {} | {}",
        display_cell(&game.board[0], expiring_piece, 0),
        display_cell(&game.board[1], expiring_piece, 1),
        display_cell(&game.board[2], expiring_piece, 2),
        display_cell(&game.board[3], expiring_piece, 3),
    );
    println!("---+---+---+---");
    println!(
        " {} | {} | {} | {}",
        display_cell(&game.board[4], expiring_piece, 4),
        display_cell(&game.board[5], expiring_piece, 5),
        display_cell(&game.board[6], expiring_piece, 6),
        display_cell(&game.board[7], expiring_piece, 7),
    );
    println!("---+---+---+---");
    println!(
        " {} | {} | {} | {}",
        display_cell(&game.board[8], expiring_piece, 8),
        display_cell(&game.board[9], expiring_piece, 9),
        display_cell(&game.board[10], expiring_piece, 10),
        display_cell(&game.board[11], expiring_piece, 11),
    );
    println!("---+---+---+---");
    println!(
        " {} | {} | {} | {}",
        display_cell(&game.board[12], expiring_piece, 12),
        display_cell(&game.board[13], expiring_piece, 13),
        display_cell(&game.board[14], expiring_piece, 14),
        display_cell(&game.board[15], expiring_piece, 15),
    );
    println!();
}

fn display_cell(cell: &Option<Player>, expiring_piece: Option<u8>, index: u8) -> char {
    let is_expiring_piece = expiring_piece == Some(index);

    match cell {
        None => ' ',
        Some(Player::X) => {
            if is_expiring_piece {
                'x'
            } else {
                'X'
            }
        }
        Some(Player::O) => {
            if is_expiring_piece {
                'o'
            } else {
                'O'
            }
        }
    }
}
