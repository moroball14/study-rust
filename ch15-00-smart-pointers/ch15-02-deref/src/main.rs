use std::ops::Deref;

fn main() {
    let m = MyBox::new(String::from("Rust"));
    // derefを呼び出すことで、 &MyBox<String>を&Stringに変換できる
    hello(&m);

    let x = 5;
    // let y = &x;
    // 上記は以下のように書き換えられる
    // let y = Box::new(x);

    let y = MyBox::new(x);

    assert_eq!(5, x);
    // この時、実際はこのコードを走らせている
    // *(y.deref())
    // Derefトレイトを実装しているので、以下のように書ける
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T>  {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
impl<T> MyBox<T>  {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name)
}