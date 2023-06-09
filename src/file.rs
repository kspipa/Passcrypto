use std::fs;
use std::io::prelude::*;
use dirs;

pub fn create_new_file(path : String){
    fs::File::create(path).unwrap();
}
pub fn write_into(data : Vec<u8>, path : String){
    let mut file = fs::File::create(path).unwrap();
    file.write_all(data.as_slice()).unwrap();
}
pub fn read_from(path : String) -> Vec<u8>{
    let mut t = vec![0];
    t.remove(0);
    let mut file = fs::File::open(path).unwrap();
    file.read_to_end(&mut t).unwrap();
    return t;
}
pub fn write_to_log(data : &str){
    let old  = read_from(format!("{}/log", get_path_to_passs()));
    let mut file = fs::File::create(format!("{}/log", get_path_to_passs())).unwrap();
    let newdata = ["\n", data].concat();
    file.write([old , newdata.as_bytes().to_vec()].concat().as_slice());
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
    while t != 255 {
        if check_file(format!("{}/{}.ps", dir, t)){
            res.push(t);
        }
        else {
            t += 1;
            continue;
        }
        t += 1;
    }
    return res;
}
pub fn mkdir(path_with_name : String){
    fs::create_dir(path_with_name).unwrap();
}
pub fn check_dir(path : String) -> bool{
    let nigger = match fs::create_dir(path.clone()) {
        Ok(_) => {rmdir(path);false},
        Err(_) => {true}
    };
    return nigger;
}
pub fn rmdir(path : String){
    fs::remove_dir(path).unwrap();
}
pub fn rmfile(path : String){
    fs::remove_file(path).unwrap();
}
pub fn get_path_to_passs() -> String{
    return format!("{}/.passs", dirs::download_dir().unwrap().to_str().unwrap().to_string());
}
