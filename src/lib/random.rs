use std::sync::mpsc::Sender;
use std::time::Instant;

use rand::{seq::SliceRandom, thread_rng};

use super::board::{Board, Token};

// Start a round between two fictitious players who play random moves
pub fn round(rand_first_player: bool) -> Token {
	let mut board = Board::new();
	// randomly choose the first player or not
	let mut current_player =
		if rand_first_player {
			[Token::Red, Token::Yellow]
				.choose(&mut thread_rng())
				.unwrap()
				.to_owned()
		} else {
			Token::Red
		};

	while !board.is_full() && !(board.check_winner() != Token::Empty) {
		board.random_stroke(current_player);
		current_player =
			if current_player == Token::Red {
				Token::Yellow
			} else {
				Token::Red
			};
	}
	// returns the winner of the game
	board.check_winner()
}

// Launches a number of rounds between two fictitious players who play random moves
fn rounds(number_rounds: u64, tx: Sender<[(Token, u64); 3]>, rand_first_player: bool) {
	let mut res: [(Token, u64); 3] = [(Token::Red, 0), (Token::Yellow, 0), (Token::Empty, 0)];
	for _i in 0..number_rounds {
		match round(rand_first_player) {
			Token::Red => res[0].1 += 1,
			Token::Yellow => res[1].1 += 1,
			Token::Empty => res[2].1 += 1,
		};
	}
	tx.send(res).unwrap();
}

// Throws threads that will throw a certain number of rounds
pub fn game_session(
	number_thread: u32,
	number_rounds: u64,
	rand_first_player: bool,
) -> [(Token, u64); 3] {
	let total_rounds = number_rounds * number_thread as u64;
	let mut res = [(Token::Red, 0), (Token::Yellow, 0), (Token::Empty, 0)];

	let now = Instant::now();

	let mut children = vec![];
	let (tx, rx) = std::sync::mpsc::channel();
	for _i in 0..number_thread {
		let tx_copy = std::sync::mpsc::Sender::clone(&tx);
		children.push(std::thread::spawn(move || {
			rounds(number_rounds, tx_copy, rand_first_player)
		}));
	}
	std::mem::drop(tx);
	for tmp in rx {
		res[0].1 += tmp[0].1;
		res[1].1 += tmp[1].1;
		res[2].1 += tmp[2].1;
	}
	let exec_time = now.elapsed().as_millis();
	println!(
		"finished after {} milliseconds or {:.2} seconds or {:.2} minutes.",
		exec_time,
		exec_time as f64 / 1000.0,
		exec_time as f64 / 1000.0 / 60.0
	);
	println!(
		"\nresult of {} {} : ",
		total_rounds,
		if total_rounds <= 1 { "game" } else { "games" }
	);
	println!(
		"\t{:.3}% victory for the red token.",
		(res[0].1 as f64) * 100.0 / (total_rounds as f64)
	);
	println!(
		"\t{:.3}% victory for the yellow token.",
		(res[1].1 as f64) * 100.0 / (total_rounds as f64)
	);
	println!(
		"\t{:.3}% draw.",
		(res[2].1 as f64) * 100.0 / (total_rounds as f64)
	);
	res
}
