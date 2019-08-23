fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    let size = std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}size is {}", guess, size);
    let bar = rand::random::<u8>();
    println!("{}", bar);
}
