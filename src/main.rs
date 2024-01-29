mod lexer;
mod predicate;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use super::lexer;

    #[test]
    fn test_get_fixnum() {
        let reader = BufReader::new("42".as_bytes());
        assert_eq!(42, lexer::get_fixnum(reader));
    }
}

