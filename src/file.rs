use std::fs;
use std::io::prelude::*;
use json::{self, JsonValue};
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
pub fn rewrite(path : String, data : Vec<u8>){
    rmfile(path.clone());
    create_new_file(path.clone());
    write_into(data, path);
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
pub fn newdb(filepath : String, key: Vec<u8>) -> Jsondb{
    create_new_file(filepath.clone());
    let data = Jsondb::new(key, filepath);
    return data;
}
pub fn parse(data: &str) -> JsonValue{
    let parseddata = json::parse(data).unwrap();
    return parseddata;
}

#[derive(Clone)]
pub struct Jsondb{
    json: JsonValue,
    posit: JsonValue,
    pub positpath: String,
    pub filepath: String,
    pub key: Vec<u8>
}
impl Jsondb{
    pub fn new(key: Vec<u8>, filepath : String) -> Self{
        let mut data = json::JsonValue::new_object();
        data["root"]["dirs"] = json::JsonValue::new_array();
        data["root"]["pass"] = json::JsonValue::new_array();
        Jsondb{json: data,posit:json::JsonValue::new_object(), filepath: filepath, key, positpath: "/".to_string()}
    }
    pub fn from(text : &str, key: Vec<u8>, filepath : String) -> Self{
        Jsondb{filepath :"".to_string(), json: json::parse(text).unwrap(),posit:json::JsonValue::new_object(), key,positpath: "/".to_string()}
    }
    pub fn add_pass(&mut self,path: &str, mut pass : JsonValue){
        let db = self.gotupath(path).unwrap();
        let f = db["pass"].len();
        db["pass"][f] = pass;
    }
    pub fn add_dir(&mut self, path : &str, name: &str){
        let db = self.gotupath(path).unwrap();
        let f = db["dirs"].len();
        db["dirs"][f] = json::JsonValue::new_object();
        db["dirs"][f]["dirs"] = json::JsonValue::new_array();
        db["dirs"][f]["pass"] = json::JsonValue::new_array();
        db["dirs"][f]["name"] = name.into();
    }

    pub fn to_string(&mut self) -> String{
        self.json.dump()
    }
    pub fn getall(&mut self, path : &str) -> Vec<JsonValue>{
        let mut res = Vec::<JsonValue>::new();
        for i in self.get_dirs(Some(path)){
            res.push(i)
        }
        for i in self.get_passs(Some(path)){
            res.push(i);
        }
        return res;
    }
    pub fn deletebypath(&mut self, path: &str){
        let size = path.split("/").count();
        let mut truepath = String::new();
        let mut t = 0;
        let mut name = "";
        for i in path.split("/"){
            if t == size - 1{
                name = i;
                break;
            }
            truepath += &format!("/{}", i);
            t += 1;
        }
        let mut nn = self.gotupath(&truepath).unwrap();
        for d in 0..nn.len(){
            if nn["pass"][d]["name"] == name{
                nn["pass"].array_remove(d);
            }
        }
    }
    pub fn get_passs(&mut self, path: Option<&str>) -> Vec<JsonValue>{
        let mut db = &mut JsonValue::new_object();
        if path.is_some(){
            db = self.gotupath(path.unwrap()).unwrap();
        }
        else {
            db = &mut self.posit;
        }
        let l = self.gotupath(path.unwrap()).unwrap();
        let mut res = Vec::<JsonValue>::new();
        for i in 0..l["pass"].len(){
            res.push(l["pass"][i].clone())
        }
        return res;
    }
    pub fn get_dirs(&mut self, path: Option<&str>) -> Vec<JsonValue>{
        let mut db = &mut JsonValue::new_object();
        if path.is_some(){
            db = self.gotupath(path.unwrap()).unwrap();
        }
        else {
            db = &mut self.posit;
        }
        
        let mut res = Vec::<JsonValue>::new();
        for i in 0..db["dirs"].len(){
            res.push(db["dirs"][i].clone());
        }
        return res;
    }
    fn gotupath(&mut self, path : &str) -> Option<&mut JsonValue>{
        let db = &mut self.json;
        let mut startpos = &mut db["root"];
        let len = path.split("/").count();
        if path == "/"{
            return Some(startpos);
        }
        for (j, i) in path.split('/').enumerate(){
            if i.find(".ps").is_some(){
                for g in 0..startpos["pass"].len(){
                    if startpos["pass"][g]["name"].as_str().unwrap().replace('"', "") == i{
                        let mut t = 0;
                        self.positpath = String::new();
                        for i in path.split("/"){
                            if t == len - 1{
                                break;
                            }
                            self.positpath += &format!("/{}", i); 
                            t += 1;
                        }
                        return Some(&mut startpos["pass"][g]);
                    }
                }
            }
            for d in 0..startpos["dirs"].len(){
                if startpos["dirs"][d]["name"].as_str().unwrap().replace('"', "") == i{
                    if (j + 1) == len{
                        self.positpath = String::new();
                        for i in path.split("/"){
                            self.positpath += &format!("/{}", i);
                        }
                        return Some(&mut startpos["dirs"][d]);
                    }
                    startpos = &mut startpos["dirs"][d];
                }
            }
        }
        return None;
    }

}