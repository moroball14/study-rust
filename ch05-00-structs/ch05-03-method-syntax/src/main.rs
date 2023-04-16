#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// self = Rectangleということを把握している
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 構造体のインスタンスが存在しないのでメソッドとは呼ばない。 **関連関数** と呼ぶ。
    // インスタンスの初期化時に定義されることが多い（String::fromという関連関数のように）
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle {width: 10, height: 40};
    let rect3 = Rectangle {width: 60, height: 45};
    let rect4 = Rectangle::square(6);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2?, {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?, {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4?, {}", rect1.can_hold(&rect4));
}
