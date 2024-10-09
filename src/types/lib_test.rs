#[cfg(test)]
mod tests {
    use crate::types::lib::Parser;
    use std::io::{BufRead, BufReader};
    use stringreader::StringReader;

    #[test]
    fn construct_parser() {
        let mut parser = Parser::new(BufReader::new(StringReader::new("Hello")));
        let mut contents = String::new();
        parser.buf_reader.read_line(&mut contents).unwrap();
        assert_eq!(contents, "Hello");
    }

    #[test]
    fn parse_bulk_string() {
        let mut parser = Parser::new(BufReader::new(StringReader::new("+Hello")));
        let value = parser.next().unwrap();
        match value {
            crate::types::lib::RequestPrimitive::BulkString(s) => {
                assert_eq!(s, "Hello");
            }
            _ => panic!("Expected BulkString"),
        }
    }

    #[test]
    fn parse_error() {
        let mut parser = Parser::new(BufReader::new(StringReader::new("-Error")));
        let value = parser.next().unwrap();
        match value {
            crate::types::lib::RequestPrimitive::Error(s) => {
                assert_eq!(s, "Error");
            }
            _ => panic!("Expected Error"),
        }
    }

    #[test]
    fn parse_integer() {
        let mut parser = Parser::new(BufReader::new(StringReader::new(":123")));
        let value = parser.next().unwrap();
        match value {
            crate::types::lib::RequestPrimitive::Integer(i) => {
                assert_eq!(i, 123);
            }
            _ => panic!("Expected Integer"),
        }
    }

    #[test]
    fn parse_array() {
        let mut parser = Parser::new(BufReader::new(StringReader::new("*2\r\n+Hello\r\n+World")));
        let value = parser.next().unwrap();
        match value {
            crate::types::lib::RequestPrimitive::Array(a) => {
                assert_eq!(a.elements.len(), 2);
                match &a.elements[0] {
                    crate::types::lib::RequestPrimitive::BulkString(s) => {
                        assert_eq!(s, "Hello");
                    }
                    _ => panic!("Expected BulkString"),
                }
                match &a.elements[1] {
                    crate::types::lib::RequestPrimitive::BulkString(s) => {
                        assert_eq!(s, "World");
                    }
                    _ => panic!("Expected BulkString"),
                }
            }
            _ => panic!("Expected Array"),
        }
    }
    #[test]
    fn parse_array_bulk_strings() {
        let mut parser = Parser::new(BufReader::new(StringReader::new(
            "*2\r\n$5\r\nHello\r\n+World",
        )));
        let value = parser.next().unwrap();
        match value {
            crate::types::lib::RequestPrimitive::Array(a) => {
                assert_eq!(a.elements.len(), 2);
                match &a.elements[0] {
                    crate::types::lib::RequestPrimitive::BulkString(s) => {
                        assert_eq!(s, "Hello");
                    }
                    _ => panic!("Expected BulkString"),
                }
                match &a.elements[1] {
                    crate::types::lib::RequestPrimitive::BulkString(s) => {
                        assert_eq!(s, "World");
                    }
                    _ => panic!("Expected BulkString"),
                }
            }
            _ => panic!("Expected Array"),
        }
    }
}
