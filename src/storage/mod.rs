pub(crate) mod memory;

#[allow(dead_code)]
pub(crate) trait Storage {
    fn get(&self, key: Vec<u8>) -> Option<Vec<u8>>;
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>);
    fn del(&mut self, key: Vec<u8>);
}
