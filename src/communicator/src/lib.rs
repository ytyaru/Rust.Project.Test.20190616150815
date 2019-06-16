//mod adder;
pub mod adder;
pub fn get_message() -> String { String::from("Hello world !!") }
/*
pub fn add_two(a: i32) -> i32 {             // public
  internal_adder(a, 2)
}
fn internal_adder(a: i32, b: i32) -> i32 {  // private
  a + b
}
*/
// 単体テスト
#[cfg(test)]
mod tests {
    use super::*; // error[E0425]: cannot find function `get_message` in this scope
    #[test]
    fn it_works() { assert_eq!(2 + 2, 4); }
    // テスト用関数名が対象関数名とおなじ`get_message`だとエラー。error[E0308]: mismatched types
    #[test]
//    fn get_message() {
    fn test_get_message() {
        assert_eq!(get_message(), String::from("Hello world !!"));
    }
    /*
    // pubでない非公開メソッドも単体テストできる
    fn test_internal_adder() {
        assert_eq!(internal_adder(2, 3), 5);
    }
    */
}
