pub fn welcome() {
    println!("Welcome")
}

pub fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_welcome() {
        welcome();
        assert_eq!(3, sum(1, 2))
    }
}