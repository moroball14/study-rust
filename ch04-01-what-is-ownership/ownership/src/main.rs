fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    // 変数とデータの相互作用法: ムーブ
    let x = 5;
    let y = x;
    // 整数のようなコンパイル時にキチのサイズを持つ型は、
    // スタック上に保持されるので、実際の値をコピーするのも高速だからムーブされない
    println!("{}, {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // ヒープ上のポインタをs2が参照するようになり、s1は破棄
    // println!("{}, {}", s1, s2); // この地点でs1へは参照できない
    println!("{}", s2);

    println!("called takes_ownership!!");
    takes_ownership(s); // sの値が関数にムーブされ・・・
    // println!("{}", s);           // ・・・ここではもう有効ではない

    println!("called makes_copy!!");
    makes_copy(x);
    println!("{}", x);

    // 参照と借用
    let s3 = String::from("heelo");
    let len = calculate_length(&s3);
    println!("The length of '{}' is {}.", s3, len);

    // スライス型
    let mut s4 = String::from("hello"); // 例ではclearを実行させるために。mutableにしていたが、その必要はない
    let word = first_word(&s4);
    // s4.clear(); // mutableな変数だったとしても、不変として借用されているのであれば、その後可変な参照を得ることはできない!!
    println!("the first word in {} is: {}", s4, word);

    let my_string = String::from("hello world");
    // first_wordは `String` のスライスに対して機能する
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";
    // first_wordは文字列リテラルのスライスに対して機能する
    let word = first_word(&my_string_literal[..]);

    // 文字列リテラルは「それ自体すでに文字列スライスなので」スライス記法なしでも機能する
    let word = first_word(my_string_literal);
}

fn takes_ownership(some_string: String) { // some_stringがスコープに入る
    println!("{}", some_string)
} // ここでsome_stringがスコープを抜け、 `drop`が呼ばれる、後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer)
} // ここでsome_integerがスコープを抜ける。何も特別なことはしない。

fn calculate_length(s: &String) -> usize { // sはStringへの参照
    s.len()
} // ここで、sはスコープ外になる。けど、参照しているものの所有権を持っているわけではないので、何も起こらない
// 所有権って、ヒープに保持しているデータに対するもの、という理解をした
// 関数の引数に参照を取ることを `借用` と呼ぶ。
// 借用した何かを変更しようとしても、エラーが起きる。現実世界同様、借りたものはそのまま返す。

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // byte配列に変換

    for (i, &item) in bytes.iter().enumerate() { // ここがなぜ要素への参照を示さなければいけないのか、ちょっとまだわからない。。
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}