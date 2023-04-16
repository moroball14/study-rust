
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    // ベクターは同じ型の値を複数保持することができる
    // 型注釈をつけてあげる。初期化時点でベクターはどんなデータを保持するかわからない
    // 逆に値を挿入すると、推論できるようになり、型安全なコードになる
    // let v: Vec<i32> = Vec::new();
    // 以下は型注釈不要
    // let v = vec![1, 2, 3];
    // ベクターを更新する(mutにしてから)
    // v.push(5);
    // v.push(6);
    // v.push(7);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) { // getでも要素を取得できる。Optionを返して処理続けたい場合はgetが良さそう。
        Some(third) => println!("The third element is {}",third),
        None => println!("There is no third element.")
    }

    // ベクタ内の値を順に処理する
    let v2 = vec![100, 300, 500];
    for i in &v2 {
        println!("{}", i);
    }
    println!("v2 is {:#?}", v2);
    println!("------------------");

    let mut v3 = vec![100, 300, 500];
    for i in &mut v3 {
        // 可変参照が参照している値を変更するには参照外し演算子(*)を使う
        *i += 50;
    }
    println!("v3 is {:#?}", v3);

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
}
