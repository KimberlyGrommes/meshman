#![feature(old_io)]
extern crate mesh;
extern crate cgmath;

use std::old_io::BufferedReader;
use std::old_io::fs::File;
use mesh::StlFile;
use cgmath::*;

fn main() {
    let args = std::os::args();
    let meshname = args.as_slice().get(1).expect(
        "Usage: ./meshman <path/to/mesh>"
    ).as_slice();
    let meshfile = match File::open(&Path::new(meshname)) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };
    let v = Vector3::new(1.0f64, 2.0f64, 3.0f64);
    
    let mesh = StlFile::read(&mut BufferedReader::new(meshfile));
    println!("Vector3: {}", v.z);
}
