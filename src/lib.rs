pub mod board{
	#[derive(Clone, Copy, PartialEq)]
	pub enum Token { Red, Yellow, Empty }

	pub struct Board([[Token; 7]; 6]);

	impl Board {
		pub fn new() -> Board {
			Board([[Token::Empty; 7]; 6])
		}
		pub fn is_full(&self) -> bool {
			for row in self.0.iter() {
				for cell in row {
					if cell == &Token::Empty { return false; }
				}
			}
			true
		}
		pub fn check_winner(&self) -> Token {
			// Verification for each player
			for player_token in [Token::Red, Token::Yellow].iter() {
				// Check horizontally
				for row in self.0.iter() {
					let mut count: u8 = 0;
					for cell in row {
						count = if cell == player_token { count + 1 } else { 0 };
						if count >= 4 { return player_token.to_owned() }
					}
				}
				// Check vertically
				for i in 0..self.0[0].len() {
					let mut count: u8 = 0;
					for j in 0..self.0.len() {
						count = if self.0[j][i] == *player_token { count + 1 } else { 0 };
						if count >= 4{ return player_token.to_owned() }
					}
				}
				// check the diagonal that starts on the left and ends on the right
				let mut pos_row: Vec<usize> = Vec::from([1,2,3,4,5]);
				while pos_row.len() > 3 {
					let mut count = 0;
					for (col, row) in pos_row.iter().enumerate() {
						count = if self.0[*row][col] == *player_token { count + 1 } else { 0 };
						if count >= 4 { return player_token.to_owned() }
					}
					pos_row.remove(0);
				}
				let mut pos_col: Vec<usize> = Vec::from([0,1,2,3,4,5,6]);
				while pos_col.len() > 3 {
					let mut count = 0;
					for (mut row, col) in pos_col.iter().enumerate() {
						if row >= 6 { row -= 1; };
						count = if self.0[row][*col] == *player_token { count + 1 } else { 0 };
						if count >= 4 { return player_token.to_owned() }
					}
					pos_col.remove(0);
				}
				// check the diagonal which starts on the right and ends on the left
				pos_col = Vec::from([5,4,3,2,1,0]);
				while pos_col.len() > 3 {
					let mut count = 0;
					for (row, col) in pos_col.iter().enumerate() {
						count = if self.0[row][*col] == *player_token { count + 1 } else { 0 };
						if count >= 4 { return player_token.to_owned() }
					}
					pos_col.remove(0);
				}
				pos_col = Vec::from([6,5,4,3,2,1]);
				let mut tmp = 0;
				while pos_col.len() > 3 {
					let mut count = 0;
					for (row, col) in pos_col.iter().enumerate() {
						count = if self.0[row+tmp][*col] == *player_token { count + 1 } else { 0 };
						if count >= 4 { return player_token.to_owned() }
					}
					pos_col.pop();
					tmp += 1;
				}
			}
			Token::Empty
		}
		pub fn player_stroke(&mut self, token: Token, col: usize) -> Option<bool> {
			// check if the column number is valid
			if !(0..7).contains(&col) { return None; }
			// check if the column is not full
			if self.0[0][col] != Token::Empty { return Some(false); }
			// browse the column from bottom to top until you find an empty cell to be able to place the player's token there
			for i in 1..=self.0.len() {
				if self.0[self.0.len() - i][col] == Token::Empty {
					self.0[self.0.len() - i][col] = token;
					return Some(true);
				}
			}
			None
		}
	}
}

pub mod random{
	use super::board::{Token, Board};
	use rand::{seq::SliceRandom, thread_rng, Rng};
	use std::sync::mpsc::Sender;
	use std::time::Instant;

	// Start a round between two fictitious players who play random moves
	pub fn round(rand_first_player: bool) -> Token {
		let mut board = Board::new();
		// the pos_list vector contains the column numbers which are each present the number of times we can enter a token in said column
		let mut pos_list: Vec<usize> = Vec::from([0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5]);
		// randomly choose the first player or not
		let mut current_player = if rand_first_player { [Token::Red, Token::Yellow].choose(&mut thread_rng()).unwrap().to_owned() } else { Token::Red };

		while !pos_list.is_empty() && !board.is_full() && !(board.check_winner() != Token::Empty) {
			// Randomly choose an element from pos_list and remove it while using it as a parameter of player_stroke
			board.player_stroke(current_player, pos_list.remove(thread_rng().gen_range(0..pos_list.len())));
			current_player = if current_player == Token::Red { Token::Yellow } else { Token::Red };
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
	pub fn game_session(number_thread: u32, number_rounds: u64, rand_first_player: bool)  -> [(Token, u64); 3] {
		let total_rounds = number_rounds * number_thread as u64;
		let mut res: [(Token, u64); 3] = [(Token::Red, 0), (Token::Yellow, 0), (Token::Empty, 0)];

		let now = Instant::now();

		let mut children = vec![];
		let (tx, rx) = std::sync::mpsc::channel();
		for _i in 0..number_thread {
			let tx_copy = std::sync::mpsc::Sender::clone(&tx);
			children.push(std::thread::spawn(move || rounds(number_rounds, tx_copy, rand_first_player)));
		}
		std::mem::drop(tx);
		for tmp in rx{
			res[0].1 += tmp[0].1;
			res[1].1 += tmp[1].1;
			res[2].1 += tmp[2].1;
		}
		let exec_time = now.elapsed().as_millis();
		println!("finished after {} milliseconds or {:.2} seconds or {:.2} minutes.", exec_time, exec_time as f64/1000.0, exec_time as f64/1000.0/60.0);
		println!("\nresult of {} game : ", total_rounds);
		println!("\t{:.3}% victory for the red token.", (res[0].1 as f64)*100.0/(total_rounds as f64));
		println!("\t{:.3}% victory for the yellow token.", (res[1].1 as f64)*100.0/(total_rounds as f64));
		println!("\t{:.3}% draw.", (res[2].1 as f64)*100.0/(total_rounds as f64));
		res
	}
}