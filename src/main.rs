use crossterm::{
    style::{Color, SetBackgroundColor, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    cursor
};
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::{thread, time::Instant};

fn main() {
    launch_games_thread(10, 100_000, true);
}

#[derive(Clone, Copy, PartialEq)]
enum Token { Red, Yellow, Empty }

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
    fn display(&self) {
        let separation_color = Color::Rgb { r: 0, g: 100, b: 255 };

        // First row to display column numbers
        println!("\n\t{}                             {}", SetBackgroundColor(separation_color), ResetColor);
        print!("column :{} {}", SetBackgroundColor(separation_color), ResetColor);
        for i in 1..=7 {
            print!("{}{} {} {} {}", SetBackgroundColor(Color::DarkCyan), SetForegroundColor(Color::Rgb { r: 255, g: 255, b: 0 }), i, SetBackgroundColor(separation_color), ResetColor);
        }
        println!("\n\t{}                             {}", SetBackgroundColor(separation_color), ResetColor);

        // Displays the body of the board with different background color
        for row in self.body.iter(){
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
                for (mut col, row) in pos_row.iter().enumerate() {
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
fn rand_game(rand_first_player: bool) -> Token {
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
// Start a number of random games and record the number of wins for each in a table
fn launch_rounds(number_rounds: u64, rand_first_player: bool) -> [(Token, u64); 3] {
    let mut res: [(Token, u64); 3] = [(Token::Red, 0), (Token::Yellow, 0), (Token::Empty, 0)];
    for _i in 0..number_rounds {
        match rand_game(rand_first_player) {
            Token::Red => res[0].1 += 1,
            Token::Yellow => res[1].1 += 1,
            Token::Empty => res[2].1 += 1,
        };
    }
    res
}
// launch threads of the launch_rounds function then get their result and put them in a single array
fn launch_games_thread(number_thread: u32, number_rounds: u64, rand_first_player: bool) -> [(Token, u64); 3] {
    let total_rounds = number_rounds * number_thread as u64;
    let mut res: [(Token, u64); 3] = [(Token::Red, 0), (Token::Yellow, 0), (Token::Empty, 0)];

    let now = Instant::now();

    let mut children = vec![];
    for _i in 0..number_thread {
        children.push(thread::spawn(move || launch_rounds(number_rounds, rand_first_player)));
    }
    for child in children {
        let tmp = child.join().unwrap();
        res[0].1 += tmp[0].1;
        res[1].1 += tmp[1].1;
        res[2].1 += tmp[2].1;
    }
    let exec_time = now.elapsed().as_millis();
    println!("finished after {} milliseconds or {} seconds or {} minutes.", exec_time, exec_time/1000, exec_time/1000/60);
    println!("\nresult of {} game : ", total_rounds);
    println!("\t{:.3}% victory for the red token.", (res[0].1 as f64)*100.0/(total_rounds as f64));
    println!("\t{:.3}% victory for the yellow token.", (res[1].1 as f64)*100.0/(total_rounds as f64));
    println!("\t{:.3}% draw.", (res[2].1 as f64)*100.0/(total_rounds as f64));
    res
}