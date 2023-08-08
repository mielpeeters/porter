use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct Data {
    data_dir: PathBuf,
}

impl Data {
    pub fn new(data_dir: PathBuf) -> Self {
        Data { data_dir }
    }
    pub fn resource(&self, name: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(fs::read_to_string(self.data_dir.join(name))?)
    }
}
