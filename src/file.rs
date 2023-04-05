use std::fs;
use std::io::prelude::*;

fn create_new(path : String){
    fs::File::create(path).unwrap();
}
fn write_into(data : Vec<Vec<u8>>, path : String){
    let mut file = fs::File::open(path).unwrap();
    for i in data{
        file.write_all(i.as_slice());
    }
}
fn read_from(path : String, read : &mut [u8]){
    let mut file = fs::File::open(path).unwrap();
    file.read(read);
}