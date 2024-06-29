fn main() {
    let number = 3;

    // if式
    if number < 5 {
        println!("condition was true!");
    } else {
        println!("condition was false!");
    }

    // 論理値以外の値が自動的に論理値に変換されることはないため必ず論理値を条件式に与える必要がある
    if number != 0 {
        println!("number was something other than zero");
    }

    // else ifもあるが、より強力なmatch式というものがある
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // ifは式のため、let文の右辺に持ってくることが可能
    // numberの型をコンパイラは事前に把握する必要があるため、ifブロックとelseブロックで型に互換性がないといけない
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number)
}
