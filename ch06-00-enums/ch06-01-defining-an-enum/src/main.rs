// enum IpAddrKing {
//     v4,
//     v6,
// }

// enumは種類がわかるだけで値を保持していない
// 今までの知識ではおそらくstructを用いて以下の表現をしていたと思われる
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };

enum IpAddrKing {
    // 異なっても良い
    v4(u8, u8, u8, u8),
    v6(String),
}

struct Ipv4Addr {
    // --snip--
}
struct Ipv6Addr {
    // --snip--
}

// 構造体で値を定義したい場合は、以下のように列挙子に埋め込むことができる
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

struct QuitMessage; // ユニット構造体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // タプル構造体
struct ChangeColorMessage(i32, i32, i32); // タプル構造体


enum Message {
    Quit(QuitMessage),
    Move(MoveMessage),
    Write(WriteMessage),
    ChangeColor(ChangeColorMessage),
}

impl Message {
    fn call(&self) {
        // --snip--
    }
}

fn main() {
    // let four = IpAddrKing::v4;
    // let six = IpAddrKing::v6;
    // route(four);
    // route(six);

    let home = IpAddrKing::v4(127, 0, 0, 1);
    let loopback = IpAddrKing::v6(String::from("::1"));
    println!("Hello, world!");

    let m = Message::Write(WriteMessage(String::from("hello")));
    m.call();
}

// fn route(ip_type: IpAddrKing) {}