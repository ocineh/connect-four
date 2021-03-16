mod lib;

use std::{thread, time::Instant};
use std::sync::mpsc::Sender;
use lib::board::{Token, rand_game};

fn main() {
    launch_games_thread(10, 100_000, true);
}

// Start a number of random games and record the number of wins for each in a table
fn launch_rounds(number_rounds: u64, tx: Sender<[(Token, u64); 3]>, rand_first_player: bool) {
    let mut res: [(Token, u64); 3] = [(Token::Red, 0), (Token::Yellow, 0), (Token::Empty, 0)];
    for _i in 0..number_rounds {
        match rand_game(rand_first_player) {
            Token::Red => res[0].1 += 1,
            Token::Yellow => res[1].1 += 1,
            Token::Empty => res[2].1 += 1,
        };
    }
    tx.send(res);
}
// launch threads of the launch_rounds function then get their result and put them in a single array
fn launch_games_thread(number_thread: u32, number_rounds: u64, rand_first_player: bool) -> [(Token, u64); 3] {
    let total_rounds = number_rounds * number_thread as u64;
    let mut res: [(Token, u64); 3] = [(Token::Red, 0), (Token::Yellow, 0), (Token::Empty, 0)];

    let now = Instant::now();

    let mut children = vec![];
    let (tx, rx) = std::sync::mpsc::channel();
    for _i in 0..number_thread {
        let tx_copy = std::sync::mpsc::Sender::clone(&tx);
        children.push(thread::spawn(move || launch_rounds(number_rounds, tx_copy, rand_first_player)));
    }
    std::mem::drop(tx);
    for tmp in rx{
        res[0].1 += tmp[0].1;
        res[1].1 += tmp[1].1;
        res[2].1 += tmp[2].1;
    }
    let exec_time = now.elapsed().as_millis();
    println!("finished after {} milliseconds or {:.2} seconds or {:.2} minutes.", exec_time, exec_time as f64/1000.0, exec_time as f64/1000.0/60.0);
    println!("\nresult of {} game : ", total_rounds);
    println!("\t{:.3}% victory for the red token.", (res[0].1 as f64)*100.0/(total_rounds as f64));
    println!("\t{:.3}% victory for the yellow token.", (res[1].1 as f64)*100.0/(total_rounds as f64));
    println!("\t{:.3}% draw.", (res[2].1 as f64)*100.0/(total_rounds as f64));
    res
}