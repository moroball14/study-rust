#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // let coin = Coin::Quarter(UsState::Alaska);
    let coin = Coin::Nickel;
    let value = value_in_cents(&coin);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("value in cents: {}", value);
    println!("six: {:#?}", six);

    // 一つのケースしか考慮したくない場合はif letを使う
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarte from {:#?}", state);
    } else {
        count += 1;
    }
    println!("count: {}", count)
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarte from {:#?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// こんな感じで、1357の挙動しか興味ない場合は、それ以外を_でマッチさせることができる
// let some_u8_value = 0u8;
// match some_u8_value {
//     1 => println!("one"),
//     3 => println!("three"),
//     5 => println!("five"),
//     7 => println!("seven"),
//     _ => (),
// }
