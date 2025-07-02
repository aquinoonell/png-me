use std::{fs, str};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;


pub fn print (path: &str) {
    let buffer = get_bytes_from_path(path);
}


pub fn remove(path: &str, chunk_type: &str) -> std::io::Result<bool>{
    let buffer = get_bytes_from_path(path);
    let mut png = Png::try_from(buffer.as_slice()).unwrap();
}

fn get_bytes_from_path(path: &str) -> Vec<u8>{
    let mut f = File::open(path).expect("No file found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Buffer overflow");
    buffer
}
