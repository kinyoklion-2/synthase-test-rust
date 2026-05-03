pub fn hello() -> &'static str {
    "hello"
}

pub fn parse(input: &str) -> Vec<&str> {
    input.split_whitespace().filter(|s| !s.is_empty()).collect()
}
