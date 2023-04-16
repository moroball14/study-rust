# [derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    print!("The area of the rectangle is {} square pixels.", area(&rect1));

    // println!は標準で、波括弧はDisplayとして知られる整形をprintlnに指示する（Displayは基本系だけに対応し構造体は対応していないし無理に推測させない）
    // println!("rect1 is {}", rect1);
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}