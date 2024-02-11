use sha2::{Digest , Sha256};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use clipboard::{ClipboardProvider, ClipboardContext};


pub fn pad(pass : &[u8]) -> Vec<u8>{
    let mut hh: Vec<u8> = vec![0];
    let len = pass.len() + 1;
    let lastofblock = (len)%16;
    let mut t = 1;
    let mut rng = rand::thread_rng();
    hh[0] = 0;
    for i in pass{
        hh.push(*i);
        t += 1;
    }
    let pos = 16 - lastofblock;
    if len % 16 != 0 {
        for _i in 0..pos{
            hh.push(rng.gen());
        }
    }
    let the_last_hope = t as u8;
    hh[0] = the_last_hope;
    return hh;
}
pub fn unpad(mut changed_pass : Vec<u8>) -> Vec<u8>{
    let nigofall = changed_pass[0] as usize;
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
    clip.set_contents(text);
}
pub fn to_hex(data: &[u8]) -> String{
    let mut new_block:Vec<String> = vec![];
    let mut hex_str = String::from("0");
    for i in data.into_iter(){
        hex_str = format!("{:x}", &i);
        new_block.push(hex_str);
    }
    return new_block.concat();
}