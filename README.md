# Connect four

## What is it ?

This is simply my version of connect four, <s>for the moment the only way to win is to line up 4
tokens vertically or horizontally</s> you can win by lining up 4 tokens in a row, column or
diagonal.

## Build

You can build the project by running the following command:

```bash
cargo build --release
```

## Usage

You can run the game by running the following command:

```bash
cargo run -- -h # to see the help
```

## Roadmap

- [X] Check if the tray is full.
- [X] Check if there is a winner vertically and horizontally.
- [X] Check if there is a winner diagonally.
- [X] Be able to start a game where the computer plays against itself with random moves.
- [X] Be able to launch a certain number of games and collect in a table the number of victories
  corresponding to each player and the equality.
- [X] Be able to parallelize the launch of parts in threads.
- [X] To be able to play a move by entering only the column.
- [X] Being able to play a game with someone else.
- [X] To be able to play a game against the computer (but just random hits).

## License

[MIT](https://choosealicense.com/licenses/mit/)
