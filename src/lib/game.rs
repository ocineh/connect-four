use std::io;
use std::io::Write;

use crossterm::{
	cursor,
	style::{Color, ResetColor, SetForegroundColor},
	terminal::{Clear, ClearType},
};

use super::board::{Board, Token};

fn winner_message(board: &Board) {
	match board.check_winner() {
		Token::Yellow => println!("Victory for the player with the {}yellow tokens !{}", SetForegroundColor(Color::Rgb { r: 255, g: 255, b: 50 }), ResetColor),
		Token::Red => println!("Victory for the player with the {}red tokens !{}", SetForegroundColor(Color::Rgb { r: 255, g: 0, b: 0 }), ResetColor),
		Token::Empty => println!("The game ended in a draw."),
	};
}

fn ask_column(current_player: &Token) -> Result<i8, std::num::ParseIntError> {
	match current_player {
		Token::Red => print!("The player with the {}red token{} must choose a column number : ", SetForegroundColor(Color::Rgb { r: 255, g: 0, b: 0 }), ResetColor),
		Token::Yellow => print!("The player with the {}yellow token{} must choose a column number : ", SetForegroundColor(Color::Rgb { r: 255, g: 255, b: 50 }), ResetColor),
		Token::Empty => {}
	};
	std::io::stdout().flush().unwrap();

	let mut col = String::new();
	io::stdin().read_line(&mut col).expect("Error reading user input.");

	col.trim().parse::<i8>()
}

pub fn against_another_player() {
	let mut board = Board::new();
	let mut current_player = Token::Red;

	while !board.is_full() && !(board.check_winner() != Token::Empty) {
		println!("{}{}Current game.", Clear(ClearType::FromCursorUp), cursor::MoveTo(0, 0));
		board.display();

		let col: i8 = match ask_column(&current_player) {
			Ok(num) => num,
			Err(_) => continue,
		};

		// try to place the token in the column selected by the user and deal with the potential problem
		match board.player_stroke(current_player, col - 1) {
			None => continue,
			Some(t) => if !t { continue; }
		}

		current_player = if current_player == Token::Red { Token::Yellow } else { Token::Red };
	}

	println!("{}{}Party to finish.", Clear(ClearType::FromCursorUp), cursor::MoveTo(0, 0));
	board.display();
	winner_message(&board);
}

pub fn against_computer() {
	let mut board = Board::new();

	while !board.is_full() && !(board.check_winner() != Token::Empty) {
		println!("{}{}Current game.", Clear(ClearType::FromCursorUp), cursor::MoveTo(0, 0));
		board.display();

		let col: i8 = match ask_column(&Token::Red) {
			Ok(num) => num,
			Err(_) => continue,
		};

		match board.player_stroke(Token::Red, col - 1) {
			None => continue,
			Some(t) => if !t { continue; }
		}
		board.random_stroke(Token::Yellow);
	}

	println!("{}{}Party to finish.", Clear(ClearType::FromCursorUp), cursor::MoveTo(0, 0));
	board.display();
	winner_message(&board);
}
