use sha2::{Digest , Sha256};

pub fn change_pass_to_16_bytes(pass : &[u8]) -> Vec<u8>{
    let mut hh: Vec<u8> = vec![0];
    let len = pass.len() + 1;
    let lastofblock = (len)%16;
    let mut t = 1;
    hh[0] = 0;
    for i in pass{
        hh.push(*i);
        t += 1;
    }
    let pos = 16 - lastofblock;
    if len % 16 != 0 {
        for i in 0..pos{
            hh.push(1);
        }
    }
    let the_last_hope = t as u8;
    hh[0] = the_last_hope;
    return hh;
}
pub fn change_pass_from_16_bytes_to_normal(mut changed_pass : Vec<u8>) -> Vec<u8>{
    let nigofall = changed_pass[0] as usize;
    if nigofall >= changed_pass.len(){
        return changed_pass;
    }
    else {
        if changed_pass[nigofall] != 1{
            return changed_pass;
        } 
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
pub fn split_arr_into_passwords(data : Vec<u8>) -> Vec<Vec<u8>>{
    let nn = data[0] as usize;
    let mut hd = data.clone();
    let mut hh: Vec<Vec<u8>> = vec![vec![0]];
    hh.remove(0);
    let mut t = 0;
    while hd.len() != 1{
        let onepass = get_one_pass_from_arr(data.clone());
        hh.push(onepass.clone());
        for i in 0..onepass.len(){
            hd.remove(i);
        }
    }
    return hh;
}
fn get_one_pass_from_arr(passwords : Vec<u8>) -> Vec<u8>{
    let start = passwords[0] as usize;
    let mut newvec:Vec<u8> = vec![0];
    newvec.remove(0);
    for i in 0..passwords.len(){
        if i >= start && passwords[i] != 1{
            break;
        }
        newvec.push(passwords[i]);
    }
    return newvec;
}