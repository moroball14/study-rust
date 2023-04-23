extern crate adder;
// tests ディレクトリのテストは、個別のクレートであるため、各ライブラリをこの形式でimportする必要がある
// 

mod common;
// commonモジュールをインポートする

#[test]
fn it_adds_two() {
  common::setup();
    assert_eq!(4, adder::add_two(2));
}