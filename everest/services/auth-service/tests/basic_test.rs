#[cfg(test)]
mod basic_test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_string() {
        let s = String::from("hello");
        assert_eq!(s, "hello");
    }
}
