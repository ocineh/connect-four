mod board {
	use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};

	#[derive(Clone, Copy, PartialEq, Debug)]
	pub enum Token { Red, Yellow, Empty }

	pub struct Board([[Token; 7]; 6]);

	impl Board {
		pub fn new() -> Board {
			Board([[Token::Empty; 7]; 6])
		}
		pub fn display(&self) {
			let separation_color = Color::Rgb { r: 0, g: 100, b: 255 };

			// First row to display column numbers
			println!("\n\t{}                             {}", SetBackgroundColor(separation_color), ResetColor);
			print!("column :{} {}", SetBackgroundColor(separation_color), ResetColor);
			for i in 1..=7 {
				print!("{}{} {} {} {}", SetBackgroundColor(Color::DarkCyan), SetForegroundColor(Color::Rgb { r: 255, g: 255, b: 0 }), i, SetBackgroundColor(separation_color), ResetColor);
			}
			println!("\n\t{}                             {}", SetBackgroundColor(separation_color), ResetColor);

			// Displays the body of the board with different background color
			for row in self.0.iter(){
				print!("\t{} {}", SetBackgroundColor(separation_color), ResetColor);
				for cell in row {
					match cell {
						Token::Empty => print!("{}   {}", SetBackgroundColor(Color::White), ResetColor),
						Token::Yellow => print!("{}   {}", SetBackgroundColor(Color::Rgb { r: 255, g: 255, b: 50 }), ResetColor),
						Token::Red => print!("{}   {}", SetBackgroundColor(Color::Rgb { r: 255, g: 0, b: 0 }), ResetColor),
					};
					print!("{} {}", SetBackgroundColor(separation_color), ResetColor);
				}
				println!("\n\t{}                             {}", SetBackgroundColor(separation_color), ResetColor);
			}
		}
		pub fn is_full(&self) -> bool {
			for row in self.0.iter() {
				for cell in row {
					if cell == &Token::Empty { return false; }
				}
			}
			true
		}
		fn check_cell(&self, x: usize, y: usize, token: &Token) -> bool {
			match self.0.get(x) {
				None => false,
				Some(row) => {
					match row.get(y) {
						None => false,
						Some(cell) => {
							token == cell
						},
					}
				}
			}
		}
		fn check_row(&self, token: &Token) -> bool{
			let mut count = 4;
			for x in 0..6 {
				for y in 0..7 {
					count = if self.check_cell(x,y,token){ count - 1 } else { 4 };
					if count == 0 {
						return true;
					}
				}
			}
			false
		}
		fn check_column(&self, token: &Token) -> bool{
			let mut count = 4;
			for y in 0..7 {
				for x in 0..6 {
					count = if self.check_cell(x,y,token){ count - 1 } else { 4 };
					if count == 0 {
						return true;
					}
				}
			}
			false
		}
		fn check_diagonal(&self, token: &Token) -> bool{
			// LEFT => RIGHT
			for row in 0..3{
				let mut count = 4;
				let mut x = row;
				for y in 0..7 {
					count = if self.check_cell(x,y,token){ count - 1 } else { 4 };
					if count == 0 {
						return true;
					}
					x += 1;
				}
			}

			for col in 1..4 {
				let mut count = 4;
				let mut y = col;
				for x in 0..6 {
					count = if self.check_cell(x,y,token){ count - 1 } else { 4 };
					if count == 0 {
						return true;
					}
					y += 1;
				}
			}

			// RIGHT => LEFT
			for row in 0..3{
				let mut count = 4;
				let mut x = row;
				for y in (1..7).rev() {
					count = if self.check_cell(x,y,token){ count - 1 } else { 4 };
					if count == 0 {
						return true;
					}
					x += 1;
				}
			}

			for col in 3..6{
				let mut count = 4;
				let mut y = col;
				for x in 0..6 {
					count = if self.check_cell(x,y,token){ count - 1 } else { 4 };
					if count == 0 {
						return true;
					}
					if y == 0 { break }
					y -= 1;
				}
			}
			false
		}
		pub fn check_winner(&self) -> Token {
			if self.check_row(&Token::Red) || self.check_column(&Token::Red) || self.check_diagonal(&Token::Red) || self.check_diagonal(&Token::Red){
				return Token::Red;
			}
			else if self.check_row(&Token::Yellow) || self.check_column(&Token::Yellow) || self.check_diagonal(&Token::Yellow) || self.check_diagonal(&Token::Yellow){
				return Token::Yellow;
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

	#[cfg(test)]
	mod tests{
		use super::{Board, Token::*};

		#[test]
		fn empty_board_is_full(){
			assert!(!Board::new().is_full())
		}
		#[test]
		fn full_board_is_full(){
			let board = Board([[Red;7]; 6]);
			assert!(board.is_full())
		}
		#[test]
		fn check_row_winner(){
			let board = Board([
				[Empty;7],
				[Empty;7],
				[Empty;7],
				[Empty,Empty,Red,Red,Red,Red,Empty],
				[Empty;7],
				[Empty;7],
			]);
			assert_eq!(board.check_winner(), Red);
			let board = Board([
				[Empty;7],
				[Empty;7],
				[Empty;7],
				[Empty;7],
				[Empty,Empty,Empty,Red,Red,Red,Red],
				[Empty;7],
			]);
			assert_eq!(board.check_winner(), Red);
			let board = Board([
				[Empty;7],
				[Empty;7],
				[Red,Red,Red,Red,Empty,Empty,Empty],
				[Empty;7],
				[Empty;7],
				[Empty;7],
			]);
			assert_eq!(board.check_winner(), Red);
		}
		#[test]
		fn check_column_winner(){
			let board = Board([
				[Empty;7],
				[Empty,Empty,Red,Empty,Empty,Empty,Empty],
				[Empty,Empty,Red,Empty,Empty,Empty,Empty],
				[Empty,Empty,Red,Empty,Empty,Empty,Empty],
				[Empty,Empty,Red,Empty,Empty,Empty,Empty],
				[Empty;7],
			]);
			assert_eq!(board.check_winner(), Red);
			let board = Board([
				[Empty,Empty,Empty,Empty,Empty,Empty,Red],
				[Empty,Empty,Empty,Empty,Empty,Empty,Red],
				[Empty,Empty,Empty,Empty,Empty,Empty,Red],
				[Empty,Empty,Empty,Empty,Empty,Empty,Red],
				[Empty;7],
				[Empty;7],
			]);
			assert_eq!(board.check_winner(), Red);
			let board = Board([
				[Empty;7],
				[Empty;7],
				[Empty,Empty,Empty,Empty,Yellow,Empty,Empty],
				[Empty,Empty,Empty,Empty,Yellow,Empty,Empty],
				[Empty,Empty,Empty,Empty,Yellow,Empty,Empty],
				[Empty,Empty,Empty,Empty,Yellow,Empty,Empty],
			]);
			assert_eq!(board.check_winner(), Yellow);
		}
		#[test]
		fn check_diagonal_left_right(){
			let board = Board([
				[Empty;7],
				[Empty;7],
				[Red,Empty,Empty,Empty,Empty,Empty,Empty],
				[Empty,Red,Empty,Empty,Empty,Empty,Empty],
				[Empty,Empty,Red,Empty,Empty,Empty,Empty],
				[Empty,Empty,Empty,Red,Empty,Empty,Empty],
			]);
			assert_eq!(board.check_winner(), Red);
			let board = Board([
				[Empty;7],
				[Empty,Empty,Yellow,Empty,Empty,Empty,Empty],
				[Empty,Empty,Empty,Yellow,Empty,Empty,Empty],
				[Empty,Empty,Empty,Empty,Yellow,Empty,Empty],
				[Empty,Empty,Empty,Empty,Empty,Yellow,Empty],
				[Empty; 7],
			]);
			assert_eq!(board.check_winner(), Yellow);
			let board = Board([
				[Empty,Empty,Empty,Red,Empty,Empty,Empty],
				[Empty,Empty,Empty,Empty,Red,Empty,Empty],
				[Empty,Empty,Empty,Empty,Empty,Red,Empty],
				[Empty,Empty,Empty,Empty,Empty,Empty,Red],
				[Empty; 7],
				[Empty; 7],
			]);
			assert_eq!(board.check_winner(), Red);
		}
		#[test]
		fn check_diagonal_right_left(){
			let board = Board([
				[Empty; 7],
				[Empty; 7],
				[Empty,Empty,Empty,Empty,Empty,Red,Empty],
				[Empty,Empty,Empty,Empty,Red,Empty,Empty],
				[Empty,Empty,Empty,Red,Empty,Empty,Empty],
				[Empty,Empty,Red,Empty,Empty,Empty,Empty],
			]);
			assert_eq!(board.check_winner(), Red);
			let board = Board([
				[Empty,Empty,Empty,Yellow,Empty,Empty,Empty],
				[Empty,Empty,Yellow,Empty,Empty,Empty,Empty],
				[Empty,Yellow,Empty,Empty,Empty,Empty,Empty],
				[Yellow,Empty,Empty,Empty,Empty,Empty,Empty],
				[Empty; 7],
				[Empty; 7],
			]);
			assert_eq!(board.check_winner(), Yellow);
			let board = Board([
				[Empty,Empty,Empty,Empty,Empty,Empty,Red],
				[Empty,Empty,Empty,Empty,Empty,Red,Empty],
				[Empty,Empty,Empty,Empty,Red,Empty,Empty],
				[Empty,Empty,Empty,Red,Empty,Empty,Empty],
				[Empty; 7],
				[Empty; 7],
			]);
			assert_eq!(board.check_winner(), Red);
		}
	}
}

pub mod game {
	use std::io;

	use crossterm::{
		cursor,
		style::{Color, ResetColor, SetForegroundColor},
		terminal::{Clear, ClearType}
	};
	use rand::{Rng, thread_rng};

	use super::board::{Board, Token};

	fn winner_message(board: &Board){
		match board.check_winner() {
			Token::Yellow => println!("Victory for the player with the {}yellow tokens !{}", SetForegroundColor(Color::Rgb { r: 255, g: 255, b: 50 }), ResetColor),
			Token::Red => println!("Victory for the player with the {}red tokens !{}", SetForegroundColor(Color::Rgb { r: 255, g: 0, b: 0 }), ResetColor),
			Token::Empty => println!("The game ended in a draw."),
		};
	}
	pub fn against_another_player() {
		let mut board = Board::new();
		let mut current_player = Token::Red;

		while !board.is_full() && !(board.check_winner() != Token::Empty) {
			println!("{}{}Current game.", Clear(ClearType::FromCursorUp), cursor::MoveTo(0,0));
			board.display();

			match current_player {
				Token::Red => eprint!("The player with the {}red token{} must choose a column number : ", SetForegroundColor(Color::Rgb { r: 255, g: 0, b: 0 }), ResetColor),
				Token::Yellow => eprint!("The player with the {}yellow token{} must choose a column number : ", SetForegroundColor(Color::Rgb { r: 255, g: 255, b: 50 }), ResetColor),
				Token::Empty => {}
			};

			let mut col = String::new();
			// Retrieves the column number entered by the user
			io::stdin()
				.read_line(&mut col)
				.expect("Error reading user input.");

			// converted col from String to usize by handling errors related to input of something other than a number
			let col: usize = match col.trim().parse() {
				Ok(num) => num,
				Err(_) => {
					println!("Please enter a column number between 1 and 7.");
					continue;
				}
			};

			// try to place the token in the column selected by the user and deal with the potential problem
			match board.player_stroke(current_player, col - 1) {
				None => {
					println!("Please enter a column number between 1 and 7.");
					continue;
				}
				Some(t) => {
					match t {
						false => {
							println!("The column is full.");
							continue;
						}
						true => {}
					}
				}
			}

			current_player = if current_player == Token::Red { Token::Yellow } else { Token::Red };
		}

		println!("{}{}Party to finish.", Clear(ClearType::FromCursorUp), cursor::MoveTo(0,0));
		board.display();

		winner_message(&board);
	}
	pub fn against_computer() {
		let mut board = Board::new();
		let mut pos_list: Vec<usize> = Vec::from([0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5]);

		while !board.is_full() && !(board.check_winner() != Token::Empty) {
			println!("{}{}Current game.", Clear(ClearType::FromCursorUp), cursor::MoveTo(0,0));
			board.display();

			eprint!("The player with the {}red token{} must choose a column number : ", SetForegroundColor(Color::Rgb { r: 255, g: 0, b: 0 }), ResetColor);
			let mut col = String::new();
			// Retrieves the column number entered by the user
			io::stdin()
				.read_line(&mut col)
				.expect("Error reading user input.");
			// converted col from String to usize by handling errors related to input of something other than a number
			let col: usize = match col.trim().parse() {
				Ok(num) => num,
				Err(_) => {
					println!("Please enter a column number between 1 and 7.");
					continue;
				}
			};

			// try to place the token in the column selected by the user and deal with the potential problem
			match board.player_stroke(Token::Red, col - 1) {
				None => {
					println!("Please enter a column number between 1 and 7.");
					continue;
				}
				Some(t) => {
					match t {
						false => {
							println!("The column is full.");
							continue;
						}
						true => {
							pos_list.remove(pos_list.iter().enumerate().find(|&r| (r.1) == &(col-1)).unwrap().0);
						}
					}
				}
			}
			board.player_stroke(Token::Yellow, pos_list.remove(thread_rng().gen_range(0..pos_list.len())));
		}

		println!("{}{}Party to finish.", Clear(ClearType::FromCursorUp), cursor::MoveTo(0,0));
		board.display();

		winner_message(&board);
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
