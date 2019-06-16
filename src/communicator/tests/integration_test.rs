extern crate communicator; // `cargo new 名前 --lib`したときの名前を使う。src/lib.rsで`mod 名前`する。
//extern crate adder;

// 結合テスト
#[test]
fn it_adds_two() {
//    assert_eq!(4, communicator::add_two(2));
    assert_eq!(4, communicator::adder::add_two(2));
}
