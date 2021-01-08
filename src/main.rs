use crossterm::{
    style::{ Color, SetBackgroundColor, ResetColor, SetForegroundColor }
};

fn main() {
    println!("Wolecome to the connect four game.");
    let mut board = Board::new();
    board.body[0][0] = 'R';
    board.body[0][1] = 'R';
    board.body[0][2] = 'R';
    board.body[0][3] = 'R';
    board.display();
    println!("check winner = {:?}", board.check_winner());
}

struct Board {
    body: [[char; 7]; 6],
}

impl Board {
    fn new() -> Board {
        let f: char = ' ';
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
                    ' ' => print!("{}   {}", SetBackgroundColor(Color::White), ResetColor),
                    'Y' => print!("{}   {}", SetBackgroundColor(Color::Rgb {  r: 255, g: 255, b: 50 }), ResetColor),
                    'R' => print!("{}   {}", SetBackgroundColor(Color::Rgb { r: 255, g: 0, b: 0 }), ResetColor),
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
                if cell != &' ' { return false; }
            }
        }
        true
    }
    fn check_winner(&self) -> (bool, char) {
        // verifier pour chaque joueur
        for player_token in ['R', 'Y'].iter() {
            // verifie horizontalement
            for row in self.body.iter() {
                let mut count: u8 = 0;
                for cell in row {
                    count = if cell == player_token { count + 1 } else { 0 };
                    if count >= 4 { return (true, player_token.to_owned()); }
                }
            }
            // verifie verticalement
            for i in 0..self.body[0].len() {
                let mut count: u8 = 0;
                for j in 0..self.body.len() {
                    count = if self.body[j][i] == *player_token { count + 1 } else { 0 };
                    if count >= 4{ return (true, player_token.to_owned()) }
                }
            }
        }
        (false, ' ')
    }
}