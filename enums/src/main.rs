enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    // match式のパターンにいれるものはifとは違い論理値以外でもOK
    // =>演算子で区切ってパターンにおいて動作させるコードを記述する
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// このようなmatchとenumの組み合わせはよくあるパターン
// Optionは値が存在するかどうかの概念を表すenum、RustにはNullがない
// matchの中で全てのパターンが網羅されていないとコンパイルエラーになる
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    // matchのパターンが1つしかない場合、if-let構文が使える
    // 下2つのやっていることは同じ
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
