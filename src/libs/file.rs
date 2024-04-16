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
    let data = Jsondb::new(key, filepath.clone());
    rewrite(filepath, data.clone().to_string().as_bytes().to_vec());
    return data;
}
pub fn parse(data: &str) -> Result<JsonValue, json::Error>{
    let parseddata = json::parse(data);
    return parseddata;
}
pub fn getpathwithoutps(path : String, leg : usize) -> String{
    let len = path.split("/").count();
    let mut newstring = String::new();
    let mut t = 0;
    for i in path.split("/"){
        if i == ""{
            t += 1;
            continue;
        }
        if t == len -leg{
            break;
        }
        newstring += format!("/{}", i).as_str();
        t += 1
    }
    return newstring;

}
#[derive(Clone)]
pub struct Jsondb{
    json: JsonValue,
    pub positpath: String,
    pub filepath: String,
    pub key: Vec<u8>,
    pub user: String
}
impl Jsondb{
    pub fn new(key: Vec<u8>, filepath : String) -> Self{
        let mut data = json::JsonValue::new_object();
        data["users"] = json::JsonValue::new_array();
        data["root"]["dirs"] = json::JsonValue::new_array();
        data["root"]["pass"] = json::JsonValue::new_array();
        data["root"]["name"] = "".into();
        data["root"]["perms"] = json::JsonValue::new_array();
        Jsondb{json: data, filepath: filepath, key, positpath: "".to_string(), user : "".to_string()}
    }
    pub fn from(text : &str, key: Vec<u8>, filepath : String) -> Self{
        Jsondb{filepath :filepath, json: json::parse(text).unwrap(), key,positpath: "".to_string(), user : "".to_string()}
    }
    pub fn add_pass(&mut self,path: &str, mut pass : JsonValue){
        let db = self.gotupath(path).unwrap();
        let f = db["pass"].len();
        db["pass"][f] = pass;
        db["pass"][f]["perms"] = json::JsonValue::new_array();
    }
    pub fn add_dir(&mut self, path : &str, name: &str){
        let db = self.gotupath(path).unwrap();
        let f = db["dirs"].len();
        db["dirs"][f] = json::JsonValue::new_object();
        db["dirs"][f]["dirs"] = json::JsonValue::new_array();
        db["dirs"][f]["pass"] = json::JsonValue::new_array();
        db["dirs"][f]["name"] = name.into();
        db["dirs"][f]["perms"] = json::JsonValue::new_array();
    }
    pub fn add_user(&mut self, user : (String, String, String)){
        let mut res = json::JsonValue::new_object();
        res["username"] = user.0.into();
        res["password"] = user.1.into();
        res["email"] = user.2.into();
        let jsonf = &mut self.json["users"];
        let len = jsonf.len().clone();
        jsonf[len] = res;
    }
    pub fn set_perm(&mut self, path : &str, user : String, perm : &str){
        let mut ss = self.gotupath(path).unwrap();
        let len = ss["perms"].len();
        ss["perms"][len] = json::JsonValue::new_object();
        ss["perms"][len]["name"] = user.clone().into();
        ss["perms"][len]["perm"] = perm.into();
    }
    pub fn get_perm(&mut self, path : &str, user : String) -> String{
        let mut ss = self.gotupath(path).unwrap();
        let len = ss["perms"].len();
        for i in 0..len{
            if ss["perms"][i]["name"].to_string() == user{
                return ss["perms"][i]["perm"].to_string();
            }
        }
        return String::from("");
    }
    pub fn clear_perm(&mut self, path : &str){
        let mut ss = self.gotupath(path).unwrap();
        ss["perms"].clear();
    }
    pub fn getusers(&mut self) -> Vec<(String, String, String)>{
        let mut jsonf = &mut self.json["users"];
        let len = jsonf.len().clone();
        let mut ans = Vec::<(String, String, String)>::new();
        for i in 0..len{
            ans.push((jsonf[i]["username"].to_string(), jsonf[i]["password"].to_string(), jsonf[i]["email"].to_string()));
        }
        return ans;
    }
    pub fn deleteuser(&mut self, user : (String, String, String)) -> u8{
        let mut jsonf = &mut self.json["users"];
        let len = jsonf.len();
        for i in 0..len{
            let n = (jsonf[i]["username"].to_string().replace('"', ""), jsonf[i]["password"].to_string().replace('"', ""), jsonf[i]["email"].to_string().replace('"', ""));
            if n == user{
                jsonf.array_remove(i);
                return 1;
            }
        }
        return 0;
    }
    pub fn to_string(&mut self) -> String{
        self.json.dump()
    }
    pub fn getall(&mut self, path : Option<&str>) -> Option<Vec<JsonValue>>{
        let mut res = Vec::<JsonValue>::new();
        match self.get_dirs(path){
            Some(_) => (),
            None => return None
        }
        for i in self.get_dirs(path).unwrap(){
            res.push(i)
        }
        for i in self.get_passes(path).unwrap(){
            res.push(i);
        }
        return Some(res);
    }
    pub fn deletebypath(&mut self, mut path: &str, pass : bool){
        let size = path.split("/").count();
        let mut truepath = String::new();
        let mut t = 0;
        let mut name = "";
        for i in path.split("/"){
            if i == ""{
                t+=1;
                continue;
            }
            if t == size - 1{
                name = i;
                break;
            }
            truepath += &format!("/{}", i);
            t += 1;
        }
        if truepath == "/".to_string(){
            truepath = "".to_string();
        }
        let mut nn = self.gotupath(&truepath).unwrap();
        let mut rea = String::new();
        if pass{
            rea = "pass".to_string()
        }
        else{
            rea = "dirs".to_string()
        }
        for d in 0..nn[&rea].len(){
            if nn[&rea][d]["name"].to_string() == name.to_string(){
                nn[&rea].array_remove(d);
            }
        }
    }
    pub fn get_pass(&mut self, path : &str) -> Option<&mut JsonValue>{
        if path.contains(".ps"){
            let t = self.gotupath(path);
            return self.gotupath(path);
        }
        else{
            return None;
        }
    }
    pub fn get_passes(&mut self, path: Option<&str>) -> Option<Vec<JsonValue>>{
        let mut db = &mut JsonValue::new_object();
        if path.is_some(){
            db = match self.gotupath(path.unwrap()){
                Some(t) => t,
                None => return None
            }
        }
        else {
            db = self.gotupath(&self.positpath.clone()).unwrap();
        }
        let mut res = Vec::<JsonValue>::new();
        for i in 0..db["pass"].len(){
            res.push(db["pass"][i].clone())
        }
        return Some(res);
    }
    pub fn get_dirs(&mut self, path: Option<&str>) -> Option<Vec<JsonValue>>{
        let mut db = &mut JsonValue::new_object();
        if path.is_some(){
            db = match self.gotupath(path.unwrap()){
                Some(t) => t,
                None => return None
            }
        }
        else {
            db = self.gotupath(&self.positpath.clone()).unwrap();
        }
        let mut res = Vec::<JsonValue>::new();
        for i in 0..db["dirs"].len(){
            res.push(db["dirs"][i].clone());
        }
        return Some(res);
    }
    pub fn gotupath(&mut self, path : &str) -> Option<&mut JsonValue>{
        let len = path.split("/").count();
        let db = &mut self.json;
        let mut startpos = &mut db["root"];
        if path == ""{
            self.positpath = "".to_string();
            return Some(startpos);
        }
        let mut count = 0;
        for (j, i) in path.split('/').enumerate(){
            if i.contains(".ps"){
                for g in 0..startpos["pass"].len(){
                    if startpos["pass"][g]["name"].as_str().unwrap().replace('"', "") == i{
                        let mut t = 0;
                        if len == 2{
                            self.positpath = "".to_string();
                        }
                        else {
                            self.positpath = getpathwithoutps(path.to_string(), 1);
                        }
                        return Some(&mut startpos["pass"][g]);
                    }
                }
                return None;
            }
            for d in 0..startpos["dirs"].len(){
                if startpos["dirs"][d]["name"].as_str().unwrap().replace('"', "") == i{
                    if (j + 1) == len{
                        self.positpath = path.to_string();
                        return Some(&mut startpos["dirs"][d]);
                    }
                    startpos = &mut startpos["dirs"][d];
                    break;
                }
            }
        }
        return None;
    }
}
pub fn check_files_in_dir(path: &String) -> Vec<String>{
    let paths = fs::read_dir(path).unwrap();
    let mut res = Vec::<String>::new();
    for i in paths {
        res.push(i.unwrap().path().display().to_string())
    }
    return res;
}