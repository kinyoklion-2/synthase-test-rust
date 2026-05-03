pub fn greet(name: &str) -> String {
    format!("hello, {}", name)
}

pub fn parse(input: &str) -> Vec<String> {
    input.split_whitespace().filter(|s| !s.is_empty()).map(String::from).collect()
}
