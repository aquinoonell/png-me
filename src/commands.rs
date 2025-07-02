use std::fs;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;


pub fn print (path: &str) {
    let buffer = get_bytes_from_path(path);
}
