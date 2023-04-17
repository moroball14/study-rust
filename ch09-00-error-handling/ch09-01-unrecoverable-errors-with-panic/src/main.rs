fn main() {
    // こんな感じで自分以外のコードでpanicを起こす例↓
    let v = vec![1, 2, 3];

    v[99];

    // 自分のコードでpanicマクロを呼び出す例↓
    panic!("crash and burn");
}
