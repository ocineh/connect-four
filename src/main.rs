use std::io;

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
        println!("\t+---+---+---+---+---+---+---+");
        println!("colomn: | 1 | 2 | 3 | 4 | 5 | 6 | 7 |");
        println!("\t+---+---+---+---+---+---+---+");
        for row in self.body.iter(){
            print!("\t|");
            for cell in row {
                match cell {
                    ' ' => print!("   |"),
                    'R' => print!(" R |"),
                    'Y' => print!(" Y |"),
                    _ => print!(" ! "),
                };
            }
            println!("\n\t+---+---+---+---+---+---+---+");
        }
    }
}