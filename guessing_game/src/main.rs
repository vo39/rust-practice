use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // 変数のはデフォルトで不変 (immutable)
    // mut修飾子をつけると変数が可変になる (mutable)
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}