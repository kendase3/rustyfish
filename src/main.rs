
fn hello_func(greet_target: &str) -> String {
   format!("Hello, {}!", greet_target)
}

fn main() {
    println!("{}", hello_func("world"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hello() {
        let target = "Foss";
        let actual = hello_func(target);
        let expected = "Hello, Foss!".to_string();
        assert_eq!(expected, actual);
    }
}
