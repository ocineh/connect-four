# Connect four
## What is it ?
This is simply my version of connect four, for the moment the only way to win is to line up 4 tokens vertically or horizontally.
## Installation
It's a simple project done with rust you just need cargo to compile the program.
```bash
cargo build --release
```
## Usage
You can either compile it (see above) with cargo or directly launch it with cargo.
```bash
cargo run
```
## Roadmap
- [X] Create a game board and be able to display it in the terminal.
- [X] Set up a more beautiful board display with color on the terminal.
- [X] Check if the tray is full.
- [X] Check if there is a winner vertically and horizontally.
- [X] To be able to play a move by entering only the column.
- [X] Being able to play a game with someone else.
- [ ] To be able to play a game against the computer (but just random hits).
- [ ] Check if there is a winner diagonally.
- [ ] Be able to start a game where the computer plays against itself with random moves.
- [ ] Be able to launch a certain number of games and collect in a table the number of victories corresponding to each player and the equality.
- [ ] Be able to parallelize the launch of parts in threads.
## License
[MIT](https://choosealicense.com/licenses/mit/)
