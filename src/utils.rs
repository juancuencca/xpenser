use std::{
    fs::File, 
    io::{self, Write, Read},
};

pub fn create_file_write_all(file_path: &str, content: &[u8]) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content)?;

    Ok(())
}

pub fn read_file_string(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    
    Ok(file_contents)
}