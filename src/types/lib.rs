use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Array {
    pub elements: Vec<RequestPrimitive>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum RequestPrimitive {
    Integer(i64),
    BulkString(String),
    Array(Array),
    Error(String),
}

pub struct Parser<R: Read> {
    pub buf_reader: BufReader<R>,
}

impl<R: Read> Parser<R> {
    pub fn new(buf_reader: BufReader<R>) -> Parser<R> {
        Parser { buf_reader }
    }

    pub fn next(&mut self) -> Result<RequestPrimitive, String> {
        let mut buffer = String::new();
        if self.buf_reader.read_line(&mut buffer).unwrap() == 0 {
            return Err("Connection closed".to_string());
        }
        buffer = buffer.trim().to_string();
        let count = buffer.split_whitespace().count();
        log::debug!("Read line: {}", buffer);
        if count == 0 {
            return Err("Empty Command".to_string());
        }
        let first_char = buffer.chars().next().unwrap();
        match first_char {
            '+' => Ok(RequestPrimitive::BulkString(buffer.split_off(1))),
            '-' => Ok(RequestPrimitive::Error(buffer.split_off(1).clone())),
            ':' => {
                let number = buffer[1..].parse::<i64>().unwrap();
                Ok(RequestPrimitive::Integer(number))
            }
            '*' => {
                let mut elements = Vec::new();
                let number = buffer[1..].trim().parse::<i64>().unwrap();
                for _ in 0..number {
                    elements.push(self.next()?);
                }

                Ok(RequestPrimitive::Array(Array { elements }))
            }
            '$' => {
                let number = buffer[1..].parse::<i64>().unwrap();
                let mut nextpart = String::new();
                self.buf_reader.read_line(&mut nextpart).unwrap();
                nextpart = nextpart.trim().to_string();
                assert_eq!(nextpart.len(), number as usize);
                Ok(RequestPrimitive::BulkString(nextpart))
            }
            _ => Err(format!("Unknown Command: {}", buffer)),
        }
    }
}
