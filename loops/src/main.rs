fn main() {
    let mut count = 0;

    // loop: 無限ループ Ctrl+cかbreakで抜けられる
    // 'からはじまるラベルをつけて、ループに名前をつけることができる
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // while: 条件が真の間ループがまわる
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for: コレクションの各要素に対してコードを実行する
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }

    // Range型を使うと同じコードを一定の回数実行したいときに便利
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
