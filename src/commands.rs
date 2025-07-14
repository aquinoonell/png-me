use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub fn print(path: &str) {
    let buffer = get_bytes_from_path(path);
    let png = Png::try_from(buffer.as_slice()).unwrap();

    let chunk_types: Vec<String> = png
        .chunks()
        .iter()
        .map(|c| c.chunk_type().to_string())
        .collect();
    for chunk_type in chunk_types {
        println!("{}", chunk_type);
    }
}

pub fn encode(path: &str, chunk_type: &str, message: &str) -> std::io::Result<bool> {
    let buffer = get_bytes_from_path(path);
    let mut png = Png::try_from(buffer.as_slice()).unwrap();

    let i_end = png
        .remove_first_chunk("IEND")
        .expect("Unable to remove end Chunk");
    png.append_chunk(Chunk::new(
        ChunkType::from_str(chunk_type).unwrap(),
        message.as_bytes().into(),
    ));
    png.append_chunk(i_end);

    let write_path = std::path::Path::new(path);
    fs::write(write_path, png.as_bytes());
    println!("Message Encoded");
    Ok(true)
}

pub fn decode(path: &str, chunk_type: &str) -> std::io::Result<bool> {
    let buffer = get_bytes_from_path(path);
    let png = Png::try_from(buffer.as_slice()).unwrap();

    let target = png.chunk_by_type(chunk_type).expect("Unable to get");

    println!("Message is: {}", target.data_as_string().unwrap());

    Ok(true)
}

pub fn remove(path: &str, chunk_type: &str) -> std::io::Result<bool> {
    let buffer = get_bytes_from_path(path);
    let png = Png::try_from(buffer.as_slice()).unwrap();

    let target = png.chunk_by_type(chunk_type).expect("Unable to get Chunk");

    println!("Message is: {}", target.data_as_string().unwrap());

    Ok(true)
}

fn get_bytes_from_path(path: &str) -> Vec<u8> {
    let mut f = File::open(path).expect("No file found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Buffer overflow");
    buffer
}
