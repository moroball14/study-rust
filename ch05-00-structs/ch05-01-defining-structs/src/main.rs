struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// タプル構造体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };
    // これはエラーになる。user1は不変のインスタンスなので
    // user1.email = String::from("someone@example.com")
    let mut user2 = User {
        email: String::from("someone2@example.com"),
        username: String::from("some2username123"),
        active: true,
        sign_in_count: 1
    };
    // これはOK。mutの宣言をすることでuser2は可変のインスタンスになる。ただし、Rustでは一部のフィールドだけを可変にすることはできない。
    user2.email = String::from("someonemut2@example.com");

    let user3 = User {
        email: String::from("someone3@example.com"),
        username: String::from("some3username123"),
        ..user1
    };
    // ここTypeScriptのObject Spreadと同じように、user1のフィールドをuser3にコピーしているけど、
    // TypeScriptと違ってuser3のプロパティで明示的に定義しているものは上書きされない。なるほど。
    println!("user3.email: {}", user3.email);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    outputPoint(origin);
    // これ↓はエラーになる。構造体内のフィールドが同じ型であっても、ColorとPointは別の型と見なされる。
    // outputPoint(black);
}

fn build_user(email: String, username: String) -> User {
    // User {
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // }

    // フィールド名と同じ変数名を使う場合は以下のように省略できる
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn outputPoint (p: Point) {
    println!("x: {}, y: {}, z: {}", p.0, p.1, p.2);
}