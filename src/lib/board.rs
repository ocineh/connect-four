use std::convert::TryInto;

use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use rand::{thread_rng, Rng};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Token {
    Red,
    Yellow,
    Empty,
}

#[derive(Debug)]
pub struct Board([[Token; 7]; 6], Vec<i8>);

impl Board {
    pub fn new() -> Board {
        Board(
            [[Token::Empty; 7]; 6],
            vec![
                0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4,
                4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6,
            ],
        )
    }

    pub fn display(&self) {
        let separation_color = Color::Rgb {
            r: 0,
            g: 100,
            b: 255,
        };

        // First row to display column numbers
        println!(
            "\n\t{}                             {}",
            SetBackgroundColor(separation_color),
            ResetColor
        );
        print!(
            "column :{} {}",
            SetBackgroundColor(separation_color),
            ResetColor
        );
        for i in 1..=7 {
            print!(
                "{}{} {} {} {}",
                SetBackgroundColor(Color::DarkCyan),
                SetForegroundColor(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 0
                }),
                i,
                SetBackgroundColor(separation_color),
                ResetColor
            );
        }
        println!(
            "\n\t{}                             {}",
            SetBackgroundColor(separation_color),
            ResetColor
        );

        // Displays the body of the board with different background color
        for row in self.0.iter() {
            print!("\t{} {}", SetBackgroundColor(separation_color), ResetColor);
            for cell in row {
                match cell {
                    Token::Empty => print!("{}   {}", SetBackgroundColor(Color::White), ResetColor),
                    Token::Yellow => print!(
                        "{}   {}",
                        SetBackgroundColor(Color::Rgb {
                            r: 255,
                            g: 255,
                            b: 50
                        }),
                        ResetColor
                    ),
                    Token::Red => print!(
                        "{}   {}",
                        SetBackgroundColor(Color::Rgb { r: 255, g: 0, b: 0 }),
                        ResetColor
                    ),
                };
                print!("{} {}", SetBackgroundColor(separation_color), ResetColor);
            }
            println!(
                "\n\t{}                             {}",
                SetBackgroundColor(separation_color),
                ResetColor
            );
        }
    }

    pub fn is_full(&self) -> bool {
        for row in self.0.iter() {
            for cell in row {
                if cell == &Token::Empty {
                    return false;
                }
            }
        }
        true
    }

    fn check_cell(&self, x: usize, y: usize, token: &Token) -> bool {
        match self.0.get(x) {
            None => false,
            Some(row) => match row.get(y) {
                None => false,
                Some(cell) => token == cell,
            },
        }
    }
    fn check_row(&self, token: &Token) -> bool {
        let mut count = 4;
        for x in 0..6 {
            for y in 0..4 {
                count = if self.check_cell(x, y, token) {
                    count - 1
                } else {
                    4
                };
                if count == 0 {
                    return true;
                }
            }
        }
        false
    }
    fn check_column(&self, token: &Token) -> bool {
        let mut count = 4;
        for y in 0..7 {
            for x in 0..3 {
                count = if self.check_cell(x, y, token) {
                    count - 1
                } else {
                    4
                };
                if count == 0 {
                    return true;
                }
            }
        }
        false
    }
    fn check_diagonal(&self, token: &Token) -> bool {
        // LEFT => RIGHT
        for row in 0..3 {
            let mut count = 4;
            let mut x = row;
            for y in 0..7 {
                count = if self.check_cell(x, y, token) {
                    count - 1
                } else {
                    4
                };
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
                count = if self.check_cell(x, y, token) {
                    count - 1
                } else {
                    4
                };
                if count == 0 {
                    return true;
                }
                y += 1;
            }
        }

        // RIGHT => LEFT
        for row in 0..3 {
            let mut count = 4;
            let mut x = row;
            for y in (1..7).rev() {
                count = if self.check_cell(x, y, token) {
                    count - 1
                } else {
                    4
                };
                if count == 0 {
                    return true;
                }
                x += 1;
            }
        }

        for col in 3..6 {
            let mut count = 4;
            let mut y = col;
            for x in 0..6 {
                count = if self.check_cell(x, y, token) {
                    count - 1
                } else {
                    4
                };
                if count == 0 {
                    return true;
                }
                if y == 0 {
                    break;
                }
                y -= 1;
            }
        }
        false
    }
    pub fn check_winner(&self) -> Token {
        if self.check_row(&Token::Red)
            || self.check_column(&Token::Red)
            || self.check_diagonal(&Token::Red)
            || self.check_diagonal(&Token::Red)
        {
            return Token::Red;
        } else if self.check_row(&Token::Yellow)
            || self.check_column(&Token::Yellow)
            || self.check_diagonal(&Token::Yellow)
            || self.check_diagonal(&Token::Yellow)
        {
            return Token::Yellow;
        }
        Token::Empty
    }

    pub fn player_stroke(&mut self, token: Token, col: i8) -> Option<bool> {
        match col.try_into() {
            Ok(col) if col < 7 => {
                if !self.check_cell(0, col, &Token::Empty) {
                    Some(false)
                } else {
                    for row in (0..6).rev() {
                        if self.check_cell(row, col, &Token::Empty) {
                            self.0[row][col] = token;
                            self.1
                                .remove(self.1.iter().position(|&c| c == col as i8).unwrap());
                            return Some(true);
                        }
                    }
                    None
                }
            }
            _ => None,
        }
    }
    pub fn random_stroke(&mut self, token: Token) -> Option<bool> {
        match self.1.get(thread_rng().gen_range(0..self.1.len()) as usize) {
            None => None,
            Some(&col) => self.player_stroke(token, col),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Board, Token::*};

    #[test]
    fn empty_board_is_full() {
        assert!(!Board::new().is_full())
    }

    #[test]
    fn full_board_is_full() {
        let board = Board([[Red; 7]; 6], Vec::new());
        assert!(board.is_full())
    }

    #[test]
    fn check_row_winner() {
        let board = Board(
            [
                [Empty; 7],
                [Empty; 7],
                [Empty; 7],
                [Empty, Empty, Red, Red, Red, Red, Empty],
                [Empty; 7],
                [Empty; 7],
            ],
            Vec::new(),
        );
        assert_eq!(board.check_winner(), Red);
        let board = Board(
            [
                [Empty; 7],
                [Empty; 7],
                [Empty; 7],
                [Empty; 7],
                [Empty, Empty, Empty, Red, Red, Red, Red],
                [Empty; 7],
            ],
            Vec::new(),
        );
        assert_eq!(board.check_winner(), Red);
        let board = Board(
            [
                [Empty; 7],
                [Empty; 7],
                [Red, Red, Red, Red, Empty, Empty, Empty],
                [Empty; 7],
                [Empty; 7],
                [Empty; 7],
            ],
            Vec::new(),
        );
        assert_eq!(board.check_winner(), Red);
    }

    #[test]
    fn check_column_winner() {
        let board = Board(
            [
                [Empty; 7],
                [Empty, Empty, Red, Empty, Empty, Empty, Empty],
                [Empty, Empty, Red, Empty, Empty, Empty, Empty],
                [Empty, Empty, Red, Empty, Empty, Empty, Empty],
                [Empty, Empty, Red, Empty, Empty, Empty, Empty],
                [Empty; 7],
            ],
            Vec::new(),
        );
        assert_eq!(board.check_winner(), Red);
        let board = Board(
            [
                [Empty, Empty, Empty, Empty, Empty, Empty, Red],
                [Empty, Empty, Empty, Empty, Empty, Empty, Red],
                [Empty, Empty, Empty, Empty, Empty, Empty, Red],
                [Empty, Empty, Empty, Empty, Empty, Empty, Red],
                [Empty; 7],
                [Empty; 7],
            ],
            Vec::new(),
        );
        assert_eq!(board.check_winner(), Red);
        let board = Board(
            [
                [Empty; 7],
                [Empty; 7],
                [Empty, Empty, Empty, Empty, Yellow, Empty, Empty],
                [Empty, Empty, Empty, Empty, Yellow, Empty, Empty],
                [Empty, Empty, Empty, Empty, Yellow, Empty, Empty],
                [Empty, Empty, Empty, Empty, Yellow, Empty, Empty],
            ],
            Vec::new(),
        );
        assert_eq!(board.check_winner(), Yellow);
    }

    #[test]
    fn check_diagonal_left_right() {
        let board = Board(
            [
                [Empty; 7],
                [Empty; 7],
                [Red, Empty, Empty, Empty, Empty, Empty, Empty],
                [Empty, Red, Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Red, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Red, Empty, Empty, Empty],
            ],
            Vec::new(),
        );
        assert_eq!(board.check_winner(), Red);
        let board = Board(
            [
                [Empty; 7],
                [Empty, Empty, Yellow, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Yellow, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Yellow, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty, Yellow, Empty],
                [Empty; 7],
            ],
            Vec::new(),
        );
        assert_eq!(board.check_winner(), Yellow);
        let board = Board(
            [
                [Empty, Empty, Empty, Red, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Red, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty, Red, Empty],
                [Empty, Empty, Empty, Empty, Empty, Empty, Red],
                [Empty; 7],
                [Empty; 7],
            ],
            Vec::new(),
        );
        assert_eq!(board.check_winner(), Red);
    }

    #[test]
    fn check_diagonal_right_left() {
        let board = Board(
            [
                [Empty; 7],
                [Empty; 7],
                [Empty, Empty, Empty, Empty, Empty, Red, Empty],
                [Empty, Empty, Empty, Empty, Red, Empty, Empty],
                [Empty, Empty, Empty, Red, Empty, Empty, Empty],
                [Empty, Empty, Red, Empty, Empty, Empty, Empty],
            ],
            Vec::new(),
        );
        assert_eq!(board.check_winner(), Red);
        let board = Board(
            [
                [Empty, Empty, Empty, Yellow, Empty, Empty, Empty],
                [Empty, Empty, Yellow, Empty, Empty, Empty, Empty],
                [Empty, Yellow, Empty, Empty, Empty, Empty, Empty],
                [Yellow, Empty, Empty, Empty, Empty, Empty, Empty],
                [Empty; 7],
                [Empty; 7],
            ],
            Vec::new(),
        );
        assert_eq!(board.check_winner(), Yellow);
        let board = Board(
            [
                [Empty, Empty, Empty, Empty, Empty, Empty, Red],
                [Empty, Empty, Empty, Empty, Empty, Red, Empty],
                [Empty, Empty, Empty, Empty, Red, Empty, Empty],
                [Empty, Empty, Empty, Red, Empty, Empty, Empty],
                [Empty; 7],
                [Empty; 7],
            ],
            Vec::new(),
        );
        assert_eq!(board.check_winner(), Red);
    }

    #[test]
    fn check_player_stroke() {
        let mut board = Board::new();
        for col in 0..7 {
            for row in (0..6).rev() {
                assert_eq!(board.player_stroke(Red, col as i8), Some(true));
                assert!(board.check_cell(row, col, &Red));
            }
        }
        assert_eq!(board.player_stroke(Red, 0), Some(false));
        assert_eq!(board.player_stroke(Red, 5), Some(false));
    }
}
