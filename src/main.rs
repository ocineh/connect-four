use structopt::StructOpt;

use lib::*;

mod lib;

#[derive(StructOpt, Debug)]
enum Shell {
	Bash,
	Fish,
	PowerShell,
	Zsh,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "Connect four")]
/// A game where you need to connect four tokens to win, but you can only place tokens in a column.
///
/// When you place a token it will fall down to the lowest available space.
enum Cmd {
	/// Play a game against a fellow human
	Human,
	/// Play a game against the computer (random stroke)
	Computer,
	/// Play games with the computer against himself (random stroke)
	Random {
		#[structopt(short, long, default_value = "1")]
		/// The number of threads to use
		threads: u32,

		#[structopt(short, long, default_value = "100")]
		/// The number of rounds to play per thread
		rounds: u64,
	},
}

fn main() {
	let cmd: Cmd = Cmd::from_args();
	match cmd {
		Cmd::Human => against_another_player(),
		Cmd::Computer => against_computer(),
		Cmd::Random { threads, rounds } => {
			game_session(threads, rounds, true);
		}
	}
}
