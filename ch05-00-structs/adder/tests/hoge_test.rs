extern crate adder;
// tests ディレクトリのテストは、個別のクレートであるため、各ライブラリをこの形式でimportする必要がある
// 

mod dummycommon;
// commonモジュールをインポートする

#[test]
fn it_adds_two() {
  dummycommon::setup();
    assert_eq!(4, adder::add_two(2));
}