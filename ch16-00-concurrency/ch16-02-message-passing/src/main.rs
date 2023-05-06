use std::{sync::mpsc, thread, time::Duration};
// mpscはmultiple producer, single consumer
fn main() {
    // タプルを返し、一つ目が送信側、二つ目が受信側
    let (tx, rx) = mpsc::channel();

    // 送信側は複数持てる(multiple producer)
    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let val = String::from("hi");
        // sendの段階で、所有権を奪い値がムーブされる(受信側が所有権を得る)
        tx.send(val).unwrap();
        // なので以下のコードはエラーが出る
        // println!("value is {}", val);
    });

    // recvはスレッドの実行をブロックするので、送信側のロジックをコメントアウトしたら、送信されないためメインスレッドの処理が永遠に完了しない
    // そしてそれ自体は何もエラーを出さないので、気付けない
    let received = rx.recv().unwrap();

    println!("Got: {}", received);


    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
