// 構造体の定義
// 全体とフィールドに名前をつける
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    // 変数名とフィールド名が同じであれば省略記法が使える
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// タプル構造体の定義
// 全体には名前をつけるが、フィールドにつけられた名前はない
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// メソッド記法
// 構造体に対する関数のようなものを定義することができる
// 構造体の文脈の中にいるためメソッドの第一引数はselfとなる
// 所有権は特に必要なくデータだけを読み込みたい場合は&selfでよい
// メソッド内でメソッドを呼び出したインスタンスを変更したい場合は&mut selfを使用すればよい
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 他のrectangleを嵌め込めるか？
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 関連関数
    // implブロックの中にselfを引数に取らない関数を定義できる
    // 構造体の新規インスタンスを返すコンストラクタによく使われる
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn area(reactangle: Rectangle) -> u32 {
    reactangle.width * reactangle.height
}

fn main() {
    // 構造体のインスタンスの生成
    // 構造体名の後ろに、key: valueを{}で囲むことによって生成できる
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

    // 構造体更新記法を使うことで、新しいUserインスタンスにemailとusernameをセットしつつ、他の値はuser1の値を使える
    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        ..user1
    };

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("rect1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 関連関数の呼び出しは構造体名と::を使う
    let _sq = Rectangle::square(10);
}
