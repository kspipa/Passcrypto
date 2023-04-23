mod aes256;
mod pass;
mod file;
fn main(){
    let mut key = vec![0];
    if file::check_file("passs/check".to_string()){
        key = get_pass(false);
        if key[0] == 0{return;}
    }
    else{
        if file::check_file("passs/0.ps".to_string()){
            println!("check was deleted and your master password is missed\nWe make another one");
            for i in file::get_all_ps("passs".to_string()){
                file::rmdir("passs/{i}.ps".to_string());
            }
        }
        key = get_pass(true);
    }
    menu(key);
}
fn get_pass(first : bool) -> Vec<u8>{
    if first{
        file::mkdir("passs".to_string());
        println!("Set your first master password");
        let pass = rpassword::read_password().unwrap();
        let key = pass::get_hash_from_pass(pass.as_bytes());
        file::create_new_ps_file("passs/check".to_string());
        let data = encrypt_thats_all("TRUE".as_bytes().to_vec(), key.clone());
        file::write_into(aes256::concat_from_blocks_to_arr(data), "passs/check".to_string());
        return key;
    }
    else{
        println!("Print your master password, please");
        let pass = rpassword::read_password().unwrap();
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
    let mut username = String::new();
    println!("Type your username");
    std::io::stdin().read_line(&mut username).unwrap();
    println!("Type your new pass");
    let mut pass = String::new();
    let mut res = String::new();
    std::io::stdin().read_line(&mut pass).unwrap();
    println!("Type resource");
    std::io::stdin().read_line(&mut res).unwrap();
    return format!("{}:{}:{}",username, pass, res);
}
fn menu(key : Vec<u8>){
    loop{   
        println!("Ok, what do you want to do : \n    1. Write new password \n    2. Get passwords \n    3. Change password \n    4. Delete password \n    5. Quit \nSet number");
        let mut num = String::new();
        for i in std::io::stdin().lines(){ num = i.unwrap(); break;}
        let rr = num.as_str();
        match rr {
            "1" => {
                let pass = get_comapass().to_string();
                let t = file::get_all_ps("passs".to_string()).len();
                let newfilepath = format!("passs/{t}.ps");
                file::create_new_ps_file(newfilepath.clone());
                let encryptedpass = encrypt_thats_all(pass.as_bytes().to_vec(), key.clone());
                file::write_into(aes256::concat_from_blocks_to_arr(encryptedpass), newfilepath);
                println!("Password has been write sucesfully");
            }
            "2" => {
                let mut d = 0;
                for i in file::get_all_ps("passs".to_string()){
                    let data = file::read_from(format!("passs/{i}.ps"));
                    let n = decrypt_thats_all(data, key.clone());
                    println!("{d}.{:?}", pass::from_vec_to_string(n).replace("\n", ""));
                    d += 1
                }
                if d == 0{
                    println!("You have no passwords. Use menu for writing new one\n");
                }
            }
            "3" => {
                let mut str = String::new();
                println!("Print your password's number");
                std::io::stdin().read_line(&mut str).unwrap();
                let path = format!("passs/{}.ps", str).replace("\n", "");
                if file::check_file(path.clone()){
                    println!("Your old pass : {}", pass::from_vec_to_string(decrypt_thats_all(file::read_from(path.clone()), key.clone())).replace("\n", ""));
                    let pass = get_comapass().to_string();
                    let encryptedpass = encrypt_thats_all(pass.as_bytes().to_vec(), key.clone());
                    file::rmfile(path.clone());
                    file::create_new_ps_file(path.clone());
                    file::write_into(aes256::concat_from_blocks_to_arr(encryptedpass), path);
                    println!("Password has been change sucesfully");
                }
                else{
                    println!("You dont have this password");
                }
            }
            "4" => {
                let mut str = String::new();
                println!("Print your password's number");
                std::io::stdin().read_line(&mut str).unwrap();
                let path = format!("passs/{}.ps", str).replace("\n", "");
                if file::check_file(path.clone()){
                    file::rmfile(path);
                    println!("Your password succesfully deleted");
                }
                else{
                    println!("You dont have this password");
                }
            }
            "5" => {
                return;
            }
            _ => {
                println!("Set current number, please");
                continue;
            }
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
    let jj = aes256::concat_from_blocks_to_arr(newvec);
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