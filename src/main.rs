#![feature(env)]
#![feature(old_io)]
#![feature(old_path)]

extern crate mesh;
extern crate cgmath;

use std::old_io::BufferedReader;
use std::old_io::fs::File;
use mesh::StlFile;
use cgmath::*;

fn main() {

    let mut args = std::env::args();
    args.next();  // skip arg0
    let meshname = args.next().expect(
        "Usage: ./meshman <path/to/mesh>"
    );

    let meshfile = match File::open(&Path::new(meshname)) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

    let v = Vector3::new(1.0f64, 2.0f64, 3.0f64);
    
    StlFile::read(&mut BufferedReader::new(meshfile));
    println!("Vector3: {}", v.z);

}
