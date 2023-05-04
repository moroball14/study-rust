use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

use List::{Cons, Nil};

fn main() {
    // 複数人所有できるようRc::newする
    // かつ不変な値を可変にしたいので、RefCell::newで値を生成する
    let value = Rc::new(RefCell::new(5));

    // 所有者が増える際はRc::cloneを利用する
    // valueの所有者が増えた
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // b, cはvalueを所有するaを所有する
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // 可変借用し、valueの値を変更する
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    // b, cは変更されたvalueへの参照を持つので、b, c の値も5 + 10の15を保持する
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
