use std::{fs::File, io::Read};



pub fn read_file(path : &str) -> String {
    let mut f = File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    buf
}
