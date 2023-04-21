struct Point1<T>{
    x: T,
    y: T,
}
struct Point2<T, U>{
    x: T,
    y: U,
} // 数個以上使用するとコードが読みづらくなるので、それはコードを分けるサインかもしれない
// 確かに。単一責任の原則から離れてしまいそうだ。

// implの直後にTを宣言する
impl <T> Point1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// 特定の型に対してメソッドを実装することもできる
impl Point1<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl <T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        // 値が順序づけ可能な型のみしか使用できない
        // 比較を可能にするためにトレイトを持つと指定する必要があり、それについては別の章で
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34,46,26,663,34];
    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);

    println!("The largest char is {}", result);


    // これはintegerとfloatでエラーになる
    // let wont_work = Point1 { x: 5, y: 4.0},
    let will_work = Point2 { x: 5, y: 4.0};

    let p1 = Point2 { x: 5, y: 10.4};
    let p2 = Point2 { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y)
}
