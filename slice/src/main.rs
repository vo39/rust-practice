fn main() {
    let my_string = String::from("hello, world");

    // first_wordはStringのスライスに対して機能する
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello, world";

    // 文字列リテラルのスライスに対しても機能する
    let word = first_word(&my_string_literal[..]);

    // 文字列リテラルはそれ自体がすでに文字列スライスなので、スライス記法でなくても機能する
    let word = first_word(my_string_literal);

    println!("word: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // 先頭からiバイト目までのスライス
            return &s[0..i];
        }
    }

    // ..の前後に何も記述しない場合文字列全体のスライスになる
    &s[..]
}