pub mod board{
	use rand::{seq::SliceRandom, thread_rng, Rng};

	#[derive(Clone, Copy, PartialEq)]
	pub enum Token { Red, Yellow, Empty }

	struct Board { body: [[Token; 7]; 6] }

	impl Board {
		fn new() -> Board {
			let f = Token::Empty;
			Board {
				body: [
					[f, f, f, f, f, f, f],
					[f, f, f, f, f, f, f],
					[f, f, f, f, f, f, f],
					[f, f, f, f, f, f, f],
					[f, f, f, f, f, f, f],
					[f, f, f, f, f, f, f],
				],
			}
		}
		fn is_full(&self) -> bool {
			for row in self.body.iter() {
				for cell in row {
					if cell == &Token::Empty { return false; }
				}
			}
			true
		}
		fn check_winner(&self) -> Token {
			// Verification for each player
			for player_token in [Token::Red, Token::Yellow].iter() {
				// Check horizontally
				for row in self.body.iter() {
					let mut count: u8 = 0;
					for cell in row {
						count = if cell == player_token { count + 1 } else { 0 };
						if count >= 4 { return player_token.to_owned() }
					}
				}
				// Check vertically
				for i in 0..self.body[0].len() {
					let mut count: u8 = 0;
					for j in 0..self.body.len() {
						count = if self.body[j][i] == *player_token { count + 1 } else { 0 };
						if count >= 4{ return player_token.to_owned() }
					}
				}
				// check the diagonal that starts on the left and ends on the right
				let mut pos_row: Vec<usize> = Vec::from([1,2,3,4,5]);
				while pos_row.len() > 3 {
					let mut count = 0;
					for (col, row) in pos_row.iter().enumerate() {
						count = if self.body[*row][col] == *player_token { count + 1 } else { 0 };
						if count >= 4 { return player_token.to_owned() }
					}
					pos_row.remove(0);
				}
				let mut pos_col: Vec<usize> = Vec::from([0,1,2,3,4,5,6]);
				while pos_col.len() > 3 {
					let mut count = 0;
					for (mut row, col) in pos_col.iter().enumerate() {
						if row >= 6 { row -= 1; };
						count = if self.body[row][*col] == *player_token { count + 1 } else { 0 };
						if count >= 4 { return player_token.to_owned() }
					}
					pos_col.remove(0);
				}
				// check the diagonal which starts on the right and ends on the left
				pos_col = Vec::from([5,4,3,2,1,0]);
				while pos_col.len() > 3 {
					let mut count = 0;
					for (row, col) in pos_col.iter().enumerate() {
						count = if self.body[row][*col] == *player_token { count + 1 } else { 0 };
						if count >= 4 { return player_token.to_owned() }
					}
					pos_col.remove(0);
				}
				pos_col = Vec::from([6,5,4,3,2,1]);
				let mut tmp = 0;
				while pos_col.len() > 3 {
					let mut count = 0;
					for (row, col) in pos_col.iter().enumerate() {
						count = if self.body[row+tmp][*col] == *player_token { count + 1 } else { 0 };
						if count >= 4 { return player_token.to_owned() }
					}
					pos_col.pop();
					tmp += 1;
				}
			}
			Token::Empty
		}
		fn player_stroke(&mut self, token: Token, col: usize) -> Option<bool> {
			// check if the column number is valid
			if !(0..7).contains(&col) { return None; }
			// check if the column is not full
			if self.body[0][col] != Token::Empty { return Some(false); }
			// browse the column from bottom to top until you find an empty cell to be able to place the player's token there
			for i in 1..=self.body.len() {
				if self.body[self.body.len() - i][col] == Token::Empty {
					self.body[self.body.len() - i][col] = token;
					return Some(true);
				}
			}
			None
		}
	}

	// start a random game
	pub fn rand_game(rand_first_player: bool) -> Token {
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
}