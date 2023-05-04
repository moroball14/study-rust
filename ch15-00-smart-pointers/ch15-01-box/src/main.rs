use List::{Cons, Nil};

fn main() {
    // Boxはスタックに格納され、ヒープに格納されたデータへのポインタを保持する
    // mainの終わりで、ボックスはメモリから解放される(メモリの解放はボックスとヒープに格納されている値の両方に対して行われる)
    let b = Box::new(5);
    println!("b = {}", b);    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}


enum List {
    // Boxは参照を保持するので、Listのサイズがわかるようになる
    // 間接参照という
    Cons(i32, Box<List>),
    Nil
}