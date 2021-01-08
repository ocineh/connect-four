use crossterm::{
    style::{ Color, SetBackgroundColor, ResetColor, SetForegroundColor }
};

fn main() {
    println!("Wolecome to the connect four game.");
    let mut board = Board::new();
    board.player_stroke(Token::Yellow, 0);
    board.player_stroke(Token::Red, 2);
    board.player_stroke(Token::Yellow, 2);
    board.player_stroke(Token::Red, 2);
    board.display();
    println!("check winner = {}", board.check_winner().0);
}

#[derive(Clone, Copy, PartialEq)]
enum Token { Red, Yellow, Empty }

struct Board {
    body: [[Token; 7]; 6]
}

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
        print!("colomn :{} {}", SetBackgroundColor(separation_color), ResetColor);
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
                    Token::Yellow => print!("{}   {}", SetBackgroundColor(Color::Rgb {  r: 255, g: 255, b: 50 }), ResetColor),
                    Token::Red => print!("{}   {}", SetBackgroundColor(Color::Rgb { r: 255, g: 0, b: 0 }), ResetColor),
                    _ => print!("!!!"),
                };
                print!("{} {}", SetBackgroundColor(separation_color), ResetColor);
            }
            println!("\n\t{}                             {}", SetBackgroundColor(separation_color), ResetColor);
        }
    }
    fn is_full(&self) -> bool {
        for row in self.body.iter() {
            for cell in row {
                if cell != &Token::Empty { return false; }
            }
        }
        true
    }
    fn check_winner(&self) -> (bool, Token) {
        // Verification for each player
        for player_token in [Token::Red, Token::Yellow].iter() {
            // Check horizontally
            for row in self.body.iter() {
                let mut count: u8 = 0;
                for cell in row {
                    count = if cell == player_token { count + 1 } else { 0 };
                    if count >= 4 { return (true, player_token.to_owned()); }
                }
            }
            // Check vertically
            for i in 0..self.body[0].len() {
                let mut count: u8 = 0;
                for j in 0..self.body.len() {
                    count = if self.body[j][i] == *player_token { count + 1 } else { 0 };
                    if count >= 4{ return (true, player_token.to_owned()) }
                }
            }
        }
        (false, Token::Empty)
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