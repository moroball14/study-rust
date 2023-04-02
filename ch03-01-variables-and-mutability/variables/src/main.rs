use std::io;

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // const MAX_POINT: u32 = 100_000;
    // println!("The value of MAX_POINT is: {}", MAX_POINT);

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
    
    // tuple
    // 型つけて宣言する時
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 型つけなくても宣言できる
    let tup = (500, 6.4, 1);

    // 分配と呼ばれる方法で、tupleの要素取得できる
    let (x,y,z) = tup;
    println!("x is {}, y is {}, z is {}", x,y,z);
    // 番号でアクセスもできる
    let x2 = tup.0;
    let y2 = tup.1;
    let z2 = tup.2;
    println!("x2 is {}, y2 is {}, z2 is {}", x2,y2,z2);

    // array
    // 要素数が決まっているとarrayが良いらしい。決まっていないとベクター型を使うのが一般的。
    // let a: [i32;5] = [1,2,3,4,5]; // i32型の要素が5つのarray
    let a = [3;5]; // = [3,3,3,3,3]
    println!("first element is {}",a[0]);

    // 配列要素への無効なアクセス
    let array = [1,2,3,4,5];
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered wa not a number");

    let element = array[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

    print_labeled_measurement(5, 'h');
    
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);

    // 文と式
    // 文は、ただ操作を行うだけ。式は値を返す（テキストでは結果値に評価される、と表現している）
    // x + 1に;をつけると、それは文になってしまう
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
