use std::{sync::{Mutex, Arc}, thread};

fn main() {
    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();

    //     *num = 6
    // }

    // println!("m = {:?}", m);

    // counterの所有権は複数のスレッドにムーブすることはできないので、threadに渡す際にコンパイルエラー
    // let counter = Mutex::new(0);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // ArcはRcのような挙動を取り、並行な状況で安全に使用できる型
        // スレッド間でもメモリ安全に挙動を取るのがArc。
        // これだけ見るとArcだけでいいじゃんとなりがちだが、スレッド間でデータを共有する際の安全性を確保するには、パフォーマンスの犠牲が必要
        // シングルスレッドで処理するならarcを強制する必要がない
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
