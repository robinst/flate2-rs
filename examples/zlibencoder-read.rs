extern crate flate2;

use std::io::prelude::*;
use flate2::Compression;
use flate2::read::ZlibEncoder;
use std::fs::File;

// Open file and debug print the compressed contents
fn main() {
    println!("{:?}", open_hello_world().unwrap());
}

// Opens sample file, compresses the contents and returns a Vector or error
// File implements Read
fn open_hello_world() -> std::io::Result<Vec<u8>> {
    let f = File::open("examples/hello_world.txt")?;
    let mut z = ZlibEncoder::new(f, Compression::fast());
    let mut buffer = [0; 50];
    let byte_count = z.read(&mut buffer)?;
    Ok(buffer[0..byte_count].to_vec())
}
