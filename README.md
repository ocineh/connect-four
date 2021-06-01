# Connect four
## What is it ?
This is simply my version of connect four, <s>for the moment the only way to win is to line up 4 tokens vertically or horizontally</s> you can win by lining up four tokens vertically, horizontally or diagonally.
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
- [X] Check if the tray is full.
- [X] Check if there is a winner vertically and horizontally.
- [X] Check if there is a winner diagonally.
- [X] Be able to start a game where the computer plays against itself with random moves.
- [X] Be able to launch a certain number of games and collect in a table the number of victories corresponding to each player and the equality.
- [X] Be able to parallelize the launch of parts in threads.
- [X] Check if there is a winner diagonally.
## License
[MIT](https://choosealicense.com/licenses/mit/)
