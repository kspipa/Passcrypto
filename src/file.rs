use std::fs;
use std::io::prelude::*;

pub fn create_new(path : String){
    fs::File::create(path).unwrap();
}
pub fn write_into(data : Vec<Vec<u8>>, path : String){
    let mut file = fs::File::create(path).unwrap();
    for i in data{
        file.write_all(i.as_slice()).unwrap();
    }
}
pub fn read_from(path : String) -> &'static mut [u8]{
    let t = Vec::from("0".as_bytes());
    let mut file = fs::File::open(path).unwrap();
    let y = t.as_slice();
    file.read(y.as_mut()).unwrap();
    return t.as_slice().as_mut();
}