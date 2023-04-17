use std::{fs::File, io::{ErrorKind, Read, self}};

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // マッチガードと呼ばれるアームのパターンをさらに絞り込むことができる
        // refはerrorがガード条件式にムーブされないように必要
        // &じゃなくてrefを使っている理由は、18章で講義
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e)
                }
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    // // 冗長な時もあるので、unwrapを簡略化することもできる(Okならその値を、Errならpanicを呼ぶ)
    // let f2 = File::open("hello2.txt").unwrap();
    // // expectはunwrapと同じだが、unwrapのエラーメッセージをカスタマイズできる
    // let f3 = File::open("hello3.txt").expect("Failed to open hello3.txt");

    let result = match read_username_from_file() {
        Ok(s) => s,
        Err(e) => panic!("Error: {:?}", e)
    };
    println!("result: {}", result);
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");
    
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e)
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e)
    // }
    
    // 以下の?を使った記法でも同じことができる
    // ちなみに?はResult型の値を返す関数でしか使えないのでmain関数の中では使えない
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}