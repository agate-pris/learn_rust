extern crate rand;

use rand::prelude::*;

fn main() {
    println!("Guess the number!");
    let gen = || rand::thread_rng().gen_range(1, 10);
    let mine = gen();
    println!("mine: {}", mine);
    let high = {
        let high;
        loop {
            println!("Expect whether yours is higher or lower than mine.");
            let input = {
                let s = {
                    let mut s = String::new();
                    std::io::stdin().read_line(&mut s).ok();
                    s
                };
                let t = s.trim();
                ("higher".starts_with(t), "lower".starts_with(t))
            };
            assert_ne!(input, (true, true));
            if input == (false, false) {
                println!("Invalid input.");
            } else {
                high = input.0;
                break;
            }
        }
        high
    };
    let yours = gen();
    println!("yours: {}", yours);
    match (high, yours.cmp(&mine)) {
        (.., std::cmp::Ordering::Equal) => {
            println!("Draw");
        }
        (true, std::cmp::Ordering::Greater) | (false, std::cmp::Ordering::Less) => {
            println!("You win!");
        }
        (true, std::cmp::Ordering::Less) | (false, std::cmp::Ordering::Greater) => {
            println!("You lose.");
        }
    }
}
