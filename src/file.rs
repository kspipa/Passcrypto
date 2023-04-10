use std::fmt::format;
use std::fs;
use std::io::prelude::*;

pub fn create_new(path : String){
    fs::File::create(path).unwrap();
}
pub fn write_into(data : Vec<u8>, path : String){
    let old = read_from(path.clone());
    let mut file = fs::File::create(path).unwrap();
    file.write_all([old, data].concat().as_slice()).unwrap();
}
pub fn read_from(path : String) -> Vec<u8>{
    let mut t = vec![0];
    t.remove(0);
    let mut file = fs::File::open(path).unwrap();
    file.read_to_end(&mut t).unwrap();
    return t;
}
pub fn check_file(path : String) -> bool{
    let file = fs::File::open(path);
    return match file {
        Ok(_) => true,
        Err(_) => false
    };
}
pub fn get_all_ps(dir : String) -> Vec<u8>{
    let mut t = 0;
    let mut res: Vec<u8> = vec![0];
    res.remove(0);
    while check_file(format!("{dir}/{t}.ps")) {
        res.push(t);
        t += 1;
    }
    return res;
}
