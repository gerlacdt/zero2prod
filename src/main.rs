fn main() {
    println!("Hello, world!");
}

// #[allow(dead_code)]
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_um() {
        let actual = sum(1, 3);
        let expected = 4;
        assert_eq!(expected, actual);
    }
}
