#[derive(Debug)]
#[allow(dead_code)]
pub struct Array {
    pub elements: Vec<RequestPrimitive>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum RequestPrimitive {
    Integer(i64),
    BulkString(String),
    Array(Array),
    Error(String),
}

pub struct Parser {
    pub buffer: String,
}

impl Parser {
    pub fn new(string: String) -> Parser {
        Parser { buffer: string }
    }

    pub fn parse(&mut self) -> Result<RequestPrimitive, String> {
        let count = self.buffer.split_whitespace().count();
        if count == 0 {
            return Err("Empty Command".to_string());
        }
        let first_char = self.buffer.chars().next().unwrap();
        match first_char {
            '+' => Ok(RequestPrimitive::BulkString(
                self.buffer.split_off(1).clone(),
            )),
            '-' => Ok(RequestPrimitive::Error(self.buffer.split_off(1).clone())),
            ':' | '$' => {
                let number = self.buffer[1..].parse::<i64>().unwrap();
                Ok(RequestPrimitive::Integer(number))
            }
            '*' => {
                let mut elements = Vec::new();
                let mut parts = self.buffer[1..].split_whitespace();
                let array_length = parts.next().unwrap().parse::<i64>().unwrap();
                log::info!("Array Length: {}", array_length);
                for part in parts {
                    let value = self.parse_part(part)?;
                    elements.push(value);
                }
                Ok(RequestPrimitive::Array(Array { elements }))
            }
            _ => Err(format!("Unknown Command: {}", self.buffer)),
        }
    }

    pub fn parse_part(&self, part: &str) -> Result<RequestPrimitive, String> {
        let first_char = part.chars().next().unwrap();
        match first_char {
            '+' => Ok(RequestPrimitive::BulkString(part.to_string().split_off(1))),
            '-' => Ok(RequestPrimitive::Error(part.to_string().split_off(1))),
            ':' | '$' => {
                let number = part[1..].parse::<i64>().unwrap();
                Ok(RequestPrimitive::Integer(number))
            }
            _ => Ok(RequestPrimitive::BulkString(part.to_string())),
        }
    }
}
