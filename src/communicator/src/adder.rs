pub fn add_two(x: i32) -> i32 { x + 2 }

// 単体テスト
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_two() {
        assert_eq!(add_two(0), 2);
        assert_eq!(add_two(-1), 1);
        assert_eq!(add_two(1), 3);
    }
}
