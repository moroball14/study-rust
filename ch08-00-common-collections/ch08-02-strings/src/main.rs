fn output_added_string() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // s1はムーブされているのでこれ以降は使えない
    println!("{}", s3); // +演算子はaddメソッドを使用して、addメソッドは&strを受け取る
    // fn add(self, s: &str) -> String {
    // s1 がselfになるっぽいな。

    // ちなみにs2はString型なのにstrにできるのは、コンパイラが型強制してくれるから
    // add メソッド呼び出しの際、コンパイラは参照外し型強制を行う（&s2 を &s2[..] に変える）
}

fn output_tic_tac_toe() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // 複雑な文字列の連結にはformat!マクロを使う
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}

fn output_unicode_scalar_value() {
    let len1 = String::from("Hola").len();
    let len2 = String::from("Здравствуйте").len();
    println!("len1 is {}", len1);
    println!("len2 is {}", len2);
    let hello = "Здравствуйте";
    // 文字列は添え字アクセスできない(以下のコードはエラーになる)
    // let answer = &hello[0];

    // 文字列スライスは添え字アクセスできるが実行時エラーになることもある
    // helloは[0..4]はスライスできるが、 [0..1] はスライスできない
    // thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/main.rs:32:14
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // メモリの内部で文字列はUTF-8エンコードされている
    // このエラーは、UTF-8のバイト列の途中で文字列スライスを行ったために発生する
    let s = &hello[0..4];
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
    println!("{}", b);
    }
}

fn main() {
    // Rustのありうるエラーを晒す性質
    // 多くのプログラマが思っている以上に文字列が複雑なデータ構造であること
    // UTF-8
    // この三つの概念の組み合わせを新参者は意識した方が良い
    let data = "initial contents";
    let s = data.to_string();
    // or let s = "initial contents".to_string();
    // or let s = String::from("initial contents");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // push_strは所有権を奪わない
    println!("s1 is {}", s1);
    println!("s2 is {}", s2); // s2はまだ有効

    output_added_string();
    output_tic_tac_toe();
    output_unicode_scalar_value();
}
