fn main() {
    let x = plus_one(5);
    another_function(x);
    print_labeled_measurement(5, 'h');
}

// 関数の宣言位置をコンパイラは気にしないため、main関数の前でも後ろでも関数定義をしてOK
// 引数には型注釈が必須
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

// 引数は複数でも可、カンマで区切る
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// -> で戻り値の型を宣言する
// returnキーワードで関数から早期リターンをし値を返すこともできるが、ほとんどの関数は最後の式を暗黙的に返す
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}