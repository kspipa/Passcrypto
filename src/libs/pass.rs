use sha2::{Digest , Sha256};
use rand::{thread_rng, Rng, RngCore};
use rand::distributions::Alphanumeric;
use clipboard::{ClipboardProvider, ClipboardContext};
use json::JsonValue;

#[derive(Clone)]
pub struct Passcryptopass{
    title : String,
    username: String,
    password: String,
    url: String,
    notes: String
}
impl Passcryptopass{
    pub fn new() -> Self{
        return Passcryptopass{title: "".to_string(), username: "".to_string(), password: "".to_string(), url: "".to_string(), notes: "".to_string()};
    }
    pub fn from_json(json: JsonValue) -> Self{
        let mut res = Passcryptopass::new();
        res.set_title(json["name"].as_str().unwrap().replace('"', ""));
        res.set_username(json["username"].as_str().unwrap().replace('"', ""));
        res.set_password(json["password"].as_str().unwrap().replace('"', ""));
        res.set_url(json["url"].as_str().unwrap().replace('"', ""));
        res.set_notes(json["notes"].as_str().unwrap().replace('"', ""));
        return res;
    }
    pub fn to_vec(&mut self) -> Vec<String>{
        Vec::<String>::from([self.get_title(), self.get_username(), self.get_password(), self.get_url(), self.get_notes()])
    }
    pub fn from_vec(vec: Vec<String>) -> Self{
        let mut t = Passcryptopass::new();
        t.set_title(vec[0].clone());
        t.set_username(vec[1].clone());
        t.set_password(vec[2].clone());
        t.set_url(vec[3].clone());
        t.set_notes(vec[4].clone());
        return t;
    }
    pub fn to_json(&mut self) -> JsonValue{
        let mut res = json::JsonValue::new_object();
        res["name"] = self.get_title().into();
        res["username"] = self.get_username().into();
        res["password"] = self.get_password().into();
        res["url"] = self.get_url().into();
        res["notes"] = self.get_notes().into();
        return res;
    }
    pub fn get_title(&mut self) -> String{
        self.title.clone()
    }
    pub fn get_username(&mut self) -> String{
        self.username.clone()
    }
    pub fn get_password(&mut self) -> String{
        self.password.clone()
    }
    pub fn get_url(&mut self) -> String{
        self.url.clone()
    }
    pub fn get_notes(&mut self) -> String{
        self.notes.clone()
    }
    pub fn set_title(&mut self, data: String){
        self.title = data;
    }
    pub fn set_username(&mut self, data: String){
        self.username = data;
    }
    pub fn set_password(&mut self, data: String){
        self.password = data;
    }
    pub fn set_url(&mut self, data: String){
        self.url = data;
    }
    pub fn set_notes(&mut self, data: String){
        self.notes = data;
    }
    
}
pub fn pad(pass : &[u8]) -> Vec<u8>{
    let mut hh = vec![0];
    let len = pass.len() + 1;
    let lastofblock = (len)%16;
    let mut rng = rand::thread_rng();
    hh[0] = 0;
    for i in pass{
        hh.push(*i);
    }
    let pos = 16 - lastofblock;
    if len % 16 != 0 {
        for _i in 0..pos{
            hh.push(rng.gen());
        }
    }
    let the_last_hope = pos as u8;
    hh[0] = the_last_hope;
    return hh;
}
pub fn unpad(mut changed_pass : Vec<u8>) -> Vec<u8>{
    let nigofall = changed_pass.len() - changed_pass[0] as usize;
    if nigofall >= changed_pass.len(){
        return changed_pass;
    }
    changed_pass.remove(0);
    let new = changed_pass.split_at_mut(nigofall).0;
    let mut nn = Vec::from(new);
    nn.remove(nn.len() - 1);
    return nn;
}
pub fn get_hash_from_pass(pass : &[u8]) -> Vec<u8>{
    let mut nn = Sha256::new();
    nn.update(pass);
    let binding = nn.finalize();
    let fin = binding.as_slice();
    let vrc = Vec::from(fin);
    return vrc;
}
pub fn from_vec_to_string(data : Vec<u8>) -> String{
    let mut decstr = vec![String::new()];
    decstr.remove(0);
    for j in data{
        let h = j as char;
        decstr.push(h.to_string())
    }
    return decstr.concat();
}

pub fn generate_password(num: usize) -> String{
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(num)
        .map(char::from)
        .collect();
    return rand_string;
}
pub fn copy_to_clipboard(text : String){
    let mut clip : ClipboardContext = clipboard::ClipboardProvider::new().unwrap();
    let _ = clip.set_contents(text);
}
pub fn fillwithrand() -> Vec<u8>{
    let mut mas: [u8; 32] = [0; 32];
    thread_rng().fill_bytes(&mut mas);
    return mas.to_vec();
}
pub fn getpass() -> String{
    return rpassword::prompt_password("Password: ").unwrap();
}