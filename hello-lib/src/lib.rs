
pub fn hello()->String{
    return String::from("hello worldaaa");
}

#[cfg(test)]
mod test {
    use super::hello;

    #[test]
    fn test_Hello(){
        let result = hello();
        assert_eq!(result, "hello worldaaa");
    }
}