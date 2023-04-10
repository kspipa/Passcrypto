
mod aes256;
mod pass;
mod file;
fn main(){
    let mut key = vec![0];
    key.remove(0);
    if file::check_file("passs/check".to_string()){
        if !file::check_file("passs/0.ps".to_string()){
            println!("0.ps was deleted and your passwords are missed\nWe make another one");
            file::create_new("passs/0.ps".to_string());
        }
        key = get_pass(false);
        if key[0] == 0{return;}
    }
    else{
        if file::check_file("passs/0.ps".to_string()){
            println!("check was deleted and your master password is missed\nWe make another one");
            file::create_new("passs/0.ps".to_string());
        }
        key = get_pass(true);
    }
    menu(key);
}
fn get_pass(first : bool) -> Vec<u8>{
    let mut pass = String::new();
    if first{
        println!("Set your first master password");
        std::io::stdin().read_line(&mut pass).unwrap();
        let key = pass::get_hash_from_pass(pass.as_bytes());
        file::create_new("passs/check".to_string());
        let data = encrypt_thats_all("TRUE".as_bytes().to_vec(), key.clone());
        file::write_into(aes256::concat_from_blocks_to_arr(data), "passs/check".to_string());
        return key;
    }
    else{
        println!("Print your master password, please");
        std::io::stdin().read_line(&mut pass).unwrap();
        let key = pass::get_hash_from_pass(pass.as_bytes());
        if check_pass(key.clone()){
            return key;
        }
        else {
            println!("False\n");
            return get_pass(false);
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
            let pass = get_comapass().to_string();
            let t = file::get_all_ps("passs".to_string()).len();
            let newfilepath = format!("passs/{t}.ps");
            file::create_new(newfilepath.clone());
            let encryptedpass = encrypt_thats_all(pass.as_bytes().to_vec(), key.clone());
            file::write_into(aes256::concat_from_blocks_to_arr(encryptedpass), newfilepath);
            println!("Password has been write sucesfully");
        }
        else if rr == "2"{
            let mut d = 0;
            for i in file::get_all_ps("passs".to_string()){
                d += 1;
                let data = file::read_from(format!("passs/{i}.ps"));
                let n = decrypt_thats_all(data, key.clone());
                println!("{:?}", pass::from_vec_to_string(n).replace("\n", ""));
            }
            if d == 0{
                println!("You have no passwords. Use menu for writing new one\n");
            }
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
    let binding = pass::change_pass_to_16_bytes(data.as_slice());
    let newstr = aes256::spilt_into_bloks(binding);
    let mut nvec:Vec<Vec<u8>> = vec![vec![0]];
    nvec.remove(0);
    for i in newstr{
        nvec.push(aes256::encrypt_data(i.as_slice(), key.as_slice()));
    }
    return nvec;
}
fn decrypt_thats_all(data : Vec<u8>, key : Vec<u8>) -> Vec<u8>{
    let mm = aes256::spilt_into_bloks(data);
    let mut newvec = vec![vec![0 as u8]];
    newvec.remove(0);
    for i in mm{
        newvec.push(aes256::decrypt_data(i.as_slice(), key.as_slice()));
    }
    let mut jj = aes256::concat_from_blocks_to_arr(newvec);
    let yy = pass::change_pass_from_16_bytes_to_normal(jj);
    return yy;
}
pub fn check_pass(key : Vec<u8>) -> bool{
    let data = file::read_from("passs/check".to_string());
    let decinfo = decrypt_thats_all(data, key);
    let res = pass::from_vec_to_string(decinfo.to_vec());
    if res == "TRUE".to_string(){
        return true;
    }
    else {
        return false;
    }
    
}