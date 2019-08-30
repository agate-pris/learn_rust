extern crate rand;

//use rand::prelude::*;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let gen = || rand::thread_rng().gen_range(1, 10);
    loop {
        let mine = gen();
        println!("mine: {}", mine);
        println!("Expect whether yours is higher or lower than mine.");
        let mut s = String::new();
        let size = std::io::stdin().read_line(&mut s).expect("死");
        let yours = gen();
        println!("yours: {}", yours);
        match ("higher".starts_with(s.trim()), "lower".starts_with(s.trim()), yours > mine, yours < mine) {
            (true, false, true, false) | (false, true, false, true) => {
                println!("You win!");
                return;
            },
            (true, false, false, true) | (false, true, true, false) => {
                println!("You lose.");
                return;
            },
            (true, false, false, false) | (false, true, false, false) => {
                println!("Draw");
            }
            (false, false, ..) => {
                println!("Invalid input {}.", s);
            },
            (true, true, ..) | (.., true, true) => {
                // bug
            }
            _ => {},
        }
    }
}
