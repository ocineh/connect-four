mod board {
	use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};

	#[derive(Clone, Copy, PartialEq)]
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