use std::io::{BufRead, BufReader, BufWriter, Read, Write};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Array {
    pub elements: Vec<RESPType>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Map {
    pub elements: Vec<(RESPType, RESPType)>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum RESPType {
    Integer(i64),
    SimpleString(String),
    BulkString(String),
    Array(Array),
    Error(String),
    Map(Map),
}

pub struct Parser<R: Read> {
    pub buf_reader: BufReader<R>,
}

impl<R: Read> Parser<R> {
    pub fn new(buf_reader: BufReader<R>) -> Parser<R> {
        Parser { buf_reader }
    }

    pub fn next(&mut self) -> Result<RESPType, String> {
        let mut buffer = String::new();
        match self.buf_reader.read_line(&mut buffer) {
            Ok(_) => (),
            Err(e) => return Err(format!("Error reading line: {:?}", e)),
        }
        buffer = buffer.trim().to_string();
        log::debug!("Read line: {}", buffer);
        if buffer.is_empty() {
            return Err("Empty Command".to_string());
        }
        let first_char = buffer.chars().next().unwrap();
        match first_char {
            '+' => Ok(RESPType::SimpleString(buffer.split_off(1))),
            '-' => Ok(RESPType::Error(buffer.split_off(1).clone())),
            ':' => {
                let number = buffer[1..].parse::<i64>().unwrap();
                Ok(RESPType::Integer(number))
            }
            '*' => {
                let mut elements = Vec::new();
                let number = buffer[1..].trim().parse::<i64>().unwrap();
                for _ in 0..number {
                    elements.push(self.next()?);
                }

                Ok(RESPType::Array(Array { elements }))
            }
            '$' => {
                let number = buffer[1..].parse::<i64>().unwrap();
                let mut nextpart = String::new();
                self.buf_reader.read_line(&mut nextpart).unwrap();
                nextpart = nextpart.trim().to_string();
                assert_eq!(nextpart.len(), number as usize);
                Ok(RESPType::BulkString(nextpart))
            }
            '%' => {
                let number = buffer[1..].parse::<i64>().unwrap();
                let mut elements = Vec::new();
                for _ in 0..number {
                    let key = self.next()?;
                    let value = self.next()?;
                    log::debug!("Key: {:?}, Value: {:?}", key, value);
                    elements.push((key, value));
                }

                Ok(RESPType::Map(Map { elements }))
            }
            _ => Err(format!("Unknown Command: {}", buffer)),
        }
    }
}

pub struct Writer<W: Write> {
    pub buf_writer: BufWriter<W>,
}

impl<W: Write> Writer<W> {
    pub fn new(buf_writer: BufWriter<W>) -> Writer<W> {
        Writer { buf_writer }
    }

    pub fn write(&mut self, value: RESPType) -> Result<(), String> {
        match value {
            RESPType::Integer(i) => {
                self.buf_writer
                    .write_all(format!(":{}\r\n", i).as_bytes())
                    .unwrap();
            }
            RESPType::SimpleString(s) => {
                self.buf_writer
                    .write_all(format!("+{}\r\n", s).as_bytes())
                    .unwrap();
            }
            RESPType::BulkString(s) => {
                self.buf_writer
                    .write_all(format!("${}\r\n{}\r\n", s.len(), s).as_bytes())
                    .unwrap();
            }
            RESPType::Error(s) => {
                self.buf_writer
                    .write_all(format!("-ERR{}\r\n", s).as_bytes())
                    .unwrap();
            }
            RESPType::Array(a) => {
                self.buf_writer
                    .write_all(format!("*{}\r\n", a.elements.len()).as_bytes())
                    .unwrap();
                for e in a.elements {
                    self.write(e)?;
                }
            }
            RESPType::Map(m) => {
                self.buf_writer
                    .write_all(format!("%{}\r\n", m.elements.len()).as_bytes())
                    .unwrap();
                for (k, v) in m.elements {
                    self.write(k)?;
                    self.write(v)?;
                }
            }
        }

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), String> {
        let buf_str = String::from_utf8_lossy(self.buf_writer.buffer());
        log::debug!("Writing: {:?}", buf_str);

        match self.buf_writer.flush() {
            Ok(_) => (),
            Err(e) => return Err(format!("Error flushing: {:?}", e)),
        }
        Ok(())
    }
}
