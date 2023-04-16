// useキーワードを使うことでそれ以降は絶対パスを使わなくてもいい（hostingモジュールを関数のスコープに持ち込むことができる）
// 関数を使う時はその親モジュールをimportして、enumとか構造体の時はそれ自体をimportする慣習がRustにはある（まあ直感的に理解はできる）


mod front_of_house; // モジュールを定義すると、そのモジュールの中身は同じ名前のファイル（src/front_of_house.rs）から読み込まれる
pub use crate::front_of_house::hosting;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// mod front_of_house {
    

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        // 絶対パスの理論からこっち↓でもいける
        // crate::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn serve_order() {}

pub fn eat_at_restaurant() {
    // // 絶対パス
    // crate::front_of_house::hosting::add_to_waitlist();
    // // 早退パス
    // front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 別のパンにする
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // これはエラーになる。seasonal_fruitはprivate。
    // meal.seasonal_fruit = String::from("blueberries");

    // enumは公開すると全ての値が公開される
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();    
}

// 親モジュールをuseでスコープに持ち込むことで、Resultという型のスコープがはっきりする
// std::fmt::Resultとstd::io::Resultという宣言でスコープに持ち込んだら、RustはResultの区別ができなくなる
// use std::io::Result as IoResult;とかでもいい

use std::fmt;
use std::io;

// は以下と同義。こうすることで、縦に長くなりすぎない
// use std::{fmt, io};

fn function1() -> fmt::Result {
    // --snip--
    // （略）
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    // （略）
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
