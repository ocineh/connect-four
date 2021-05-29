mod lib;

use lib::random;

fn main() {
    random::game_session(15, 100_000_000, false);
}