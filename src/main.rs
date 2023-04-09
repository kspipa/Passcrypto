mod encrypt;
mod pass;
mod file;
fn main(){
    let mut key = vec![0];
    key.remove(0);
    if file::check_file("passs/check".to_string()){
        key = get_pass(false);
        if key[0] == 0{return;}
    }
    else{
        key = get_pass(true);
    }
    menu(key);
}
fn get_pass(first : bool) -> Vec<u8>{
    let mut pass = String::new();
    if first{
        println!("Set your first master password");
        std::io::stdin().read_line(&mut pass).unwrap();
        let key = pass::get_key_from_pass(pass.as_bytes());
        file::create_new("passs/0.ps".to_string());
        file::create_new("passs/check".to_string());
        let data = encrypt_thats_all("TRUE".as_bytes().to_vec(), key.clone());
        file::write_into(encrypt::concat_from_blocks_to_arr(data), "passs/check".to_string());
        return key;
    }
    else{
        println!("Print your master password, please");
        std::io::stdin().read_line(&mut pass).unwrap();
        let key = pass::get_key_from_pass(pass.as_bytes());
        if check_pass(key.clone()){
            return key;
        }
        else {
            println!("Print true password next time");
            return vec![0];
        }
    }

}
fn get_comapass() -> String{
    println!("Type your new pass");
    let mut pass = String::new();
    let mut res = String::new();
    std::io::stdin().read_line(&mut pass).unwrap();
    println!("Type resource");
    std::io::stdin().read_line(&mut res).unwrap();
    return format!("{}:{}", pass, res);
}
fn menu(key : Vec<u8>){
    loop{   
        println!("Ok, what do you want to do : \n    1. Write new password \n    2. Get passwords \n    3. Quit \nSet number");
        let mut num = String::new();
        for i in std::io::stdin().lines(){ num = i.unwrap(); break;}
        let rr = num.as_str();
        if rr == "1"{
            let pass = get_comapass();
            let encryptedpass = encrypt_thats_all(pass.as_bytes().to_vec(), key.clone());
            file::write_into(encrypt::concat_from_blocks_to_arr(encryptedpass), "passs/0.ps".to_string());
            println!("Password has been write sucesfully");
        }
        else if rr == "2"{
            let data = file::read_from("passs/0.ps".to_string());
            let decryptdata = pass::from_vec_to_string(decrypt_thats_all(data, key.clone()));
            println!("{decryptdata}");
        }
        else if rr == "3"{
            return;
        }
        else{
            println!("Set current number, please");
            continue;
        }
        
    }
}
fn encrypt_thats_all(data : Vec<u8>, key : Vec<u8>) -> Vec<Vec<u8>> {
    let binding = pass::change_pass(data.as_slice());
    let newstr = encrypt::spilt_into_bloks(binding);
    let mut nvec:Vec<Vec<u8>> = vec![vec![0]];
    nvec.remove(0);
    for i in newstr{
        nvec.push(encrypt::encrypt_data(i.as_slice(), key.as_slice()));
    }
    return nvec;
}
fn decrypt_thats_all(data : Vec<u8>, key : Vec<u8>) -> Vec<u8>{
    let nn = encrypt::spilt_into_bloks(data);
    let mut nvec = vec![vec![0]];
    nvec.remove(0);
    for i in nn{
        let kk = encrypt::decrypt_data(i.as_slice(), key.as_slice());
        nvec.push(kk);
    }
    let nig = encrypt::concat_from_blocks_to_arr(nvec);
    let changed_pass = pass::change_pass_to(nig);
    return changed_pass;
}
pub fn check_pass(key : Vec<u8>) -> bool{
    let data = file::read_from("passs/check".to_string());
    let res = pass::from_vec_to_string(decrypt_thats_all(data, key));
    if res == "TRUE".to_string(){
        return true;
    }
    else {
        return false;
    }
    
}