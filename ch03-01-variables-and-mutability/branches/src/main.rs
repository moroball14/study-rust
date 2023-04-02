fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number2 = if condition {5} else {6};
    println!("number2 is: {}", number2);

    // 評価式はbooleanでなければいけないので、以下のコードはコンパイルエラー
    // if number {
    //     println!("number was three");
    // }
}
