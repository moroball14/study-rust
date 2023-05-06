use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // 立ち上げたスレッドの処理が完了されるまで待ってから、メインスレッドの処理に移る
    handle.join().unwrap();
    
    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // メインスレッドの処理が完了しても、handleのスレッドの処理が終わるまでを待つ。
    // handle.join().unwrap()


    let v = vec![1,2,3];

    // moveキーワードを使用して、所有権を移すことでコンパイルエラーが消える
    // 所有権を新しく作成したスレッドに移さないと、メインスレッドの方で、dropなど参照している値が消える可能性がある
    // 所有権をメインスレッドでdropなどをしてしまうと、動かなくなるので、所有権を受け取る必要がある
    let handle2 = thread::spawn(move || {
        print!("Here's a vector: {:?}", v);
    });

    handle2.join().unwrap()
}
