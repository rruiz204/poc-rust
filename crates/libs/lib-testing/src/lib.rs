pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_worksss() {
        let result = add(2, 5);
        assert_eq!(result, 7);
    }
}