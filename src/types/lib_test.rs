#[cfg(test)]
mod tests {
    use crate::types::lib::Parser;

    #[test]
    fn construct_parser() {
        let parser = Parser::new("Hello".to_string());
        assert_eq!(parser.buffer, "Hello");
    }

    #[test]
    fn parse_bulk_string() {
        let mut parser = Parser::new("+Hello".to_string());
        let value = parser.parse().unwrap();
        match value {
            crate::types::lib::RequestPrimitive::BulkString(s) => {
                assert_eq!(s, "Hello");
            }
            _ => panic!("Expected BulkString"),
        }
    }

    #[test]
    fn parse_error() {
        let mut parser = Parser::new("-Error".to_string());
        let value = parser.parse().unwrap();
        match value {
            crate::types::lib::RequestPrimitive::Error(s) => {
                assert_eq!(s, "Error");
            }
            _ => panic!("Expected Error"),
        }
    }

    #[test]
    fn parse_integer() {
        let mut parser = Parser::new(":123".to_string());
        let value = parser.parse().unwrap();
        match value {
            crate::types::lib::RequestPrimitive::Integer(i) => {
                assert_eq!(i, 123);
            }
            _ => panic!("Expected Integer"),
        }
    }

    #[test]
    fn parse_array() {
        let mut parser = Parser::new("*2\r\n+Hello\r\n +World".to_string());
        let value = parser.parse().unwrap();
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
        let mut parser = Parser::new("*2\r\n$5\r\nHello\r\n+World".to_string());
        let value = parser.parse().unwrap();
        match value {
            crate::types::lib::RequestPrimitive::Array(a) => {
                assert_eq!(a.elements.len(), 3);
                match a.elements[0] {
                    crate::types::lib::RequestPrimitive::Integer(i) => {
                        assert_eq!(i, 5);
                    }
                    _ => panic!("Expected BulkString"),
                }
                match &a.elements[1] {
                    crate::types::lib::RequestPrimitive::BulkString(s) => {
                        assert_eq!(s, "Hello");
                    }
                    _ => panic!("Expected BulkString"),
                }
                match &a.elements[2] {
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
