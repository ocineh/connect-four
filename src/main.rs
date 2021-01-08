use std::io;
use crossterm::{
    style::{ Color, SetBackgroundColor, ResetColor }
};
use crossterm::style::SetForegroundColor;

fn main() {
    println!("Wolecome to the connect four game.");
    let board = Board::new();
    board.display();
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
                    'R' => print!("{}   {}", SetBackgroundColor(Color::Rgb {  r: 255, g: 255, b: 50 }), ResetColor),
                    'Y' => print!("{}   {}", SetBackgroundColor(Color::Rgb { r: 255, g: 0, b: 0 }), ResetColor),
                    _ => print!("!!!"),
                };
                print!("{} {}", SetBackgroundColor(separation_color), ResetColor);
            }
            println!("\n\t{}                             {}", SetBackgroundColor(separation_color), ResetColor);
        }
    }
}