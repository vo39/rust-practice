fn main() {
    // 浮動小数点数
    let _x = 2.0; // デフォルトはf64
    let _y: f32 = 3.0; // f32

    // 数値演算
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    let remainder = 43 % 5;
    println!("sum: {}\ndifference: {}\nproduct: {}\nquotient: {}\nfloored: {}\nremainder: {}\n", sum, difference, product, quotient, floored, remainder);

    // 論理値型
    let _f: bool = false;

    // 文字型 シングルクォートを使う
    let _c = 'z';

    // 複合型 タプル、配列
    // タプル 要素の型は異なるものでもOK
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // タプルからパターンマッチングを使用して個別の値を取り出せる(分配)
    let (_x, _y, _z) = tup;
    println!("The value of y is: {}", _y);
    // アクセスしたい値の番号をピリオドに続けて書くことでタプルの値に直接アクセスできる
    println!("The value of x is: {}", tup.0);

    // 配列 要素の型は同じでなければならない
    // 型の表記の仕方は [{type}; {elementCount}]
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // 各要素が同じ値を持つ場合は次のように初期化できる
    let a = [3; 5]; // [3, 3, 3, 3, 3] と同じ
    // 配列の要素へのアクセスは添字で行う、範囲外へのアクセスもちろんエラーとなる
    let first = arr[0];
    // let errorElement = arr[10];
    println!("The value of arr[0] is: {}", first);
}
