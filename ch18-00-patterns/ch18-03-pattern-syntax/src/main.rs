struct Point {
    x: i32,
    y: i32
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

fn main() {
    let a = 1;

    match a {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let b = 4;

    match b {
        // 範囲指定が可能
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let c = 'c';

    match c {
        // ASCII文字でも範囲指定が可能
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point {x: 0, y: 7};
    match p {
        // 構造体の一部のフィールドとの一致なども可能
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let Point { x, y } = p;

    assert_eq!(x, 0);
    assert_eq!(y, 7);


    let s = Some(5);
    let h = 10;

    match s {
        // 50だったよ
        Some(50) => println!("Got 50"),
        // マッチしたよ
        Some(h) => println!("Matched, h = {:?}", h),
        // 既定のケース
        _ => println!("Default case, s = {:?}", s),
    }

    // 最後にはs = {}, h = {}
    println!("at the end: s = {:?}, h = {:?}", s, h);

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Message::Quit")
        },
        Message::Move { x, y } => {
            println!("x: {}, y: {}", x, y);
        },
        Message::Write(s) => {
            println!("text is {}", s)
        },
        Message::ChangeColor(x, y, z) => {
            println!("x: {}, y: {}, z: {}", x, y, z);
        }
    }

    let robot_name = Some(String::from("Bors"));

    match robot_name {
        // refをつけないと、ムーブされるので、printlnのrobot_nameでコンパイルエラー
        // robot_nameを変えたい場合は、mutキーワードもつけて `*name = String::from(Another name)` みたいな感じにする
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);

}
