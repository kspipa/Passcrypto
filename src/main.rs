use cursive::Cursive;
use ui::start;

mod aes256;
mod pass;
mod file;
mod ui;
fn main(){
    let mut key = vec![0];
    let mut siv = cursive::default();
    if !file::check_file(format!("{}/.passs/checkpass", file::get_path_to_passs())){
        start(&mut siv, "Set your password".to_string());
    }
    else{
        if file::check_file(format!("{}/.passs/0.ps", file::get_path_to_passs())){
            for i in file::get_all_ps(format!("{}/.passs/", file::get_path_to_passs())){
                file::rmfile(format!("{}/.passs/{i}.ps", file::get_path_to_passs()));
            }
        }
        start(&mut siv, "Print your password".to_string());
    }
    siv.run();
}
fn get_pass() -> Vec<u8>{
    let pass = String::new();
    file::mkdir(format!("{}/.passs", file::get_path_to_passs()));
    let key = pass::get_hash_from_pass(pass.as_bytes());
    let path = format!("{}/.passs/checkpass", file::get_path_to_passs());
    file::create_new_file(path.clone());
    let data = encrypt_thats_all("TRUE".as_bytes().to_vec(), key.clone());
    file::write_into(aes256::concat_from_blocks_to_arr(data), path);
    return key;
}
fn menu(key : Vec<u8>){
    loop{   
        println!("Ok, what do you want to do : \n    1. Write new password \n    2. Get passwords \n    3. Change password \n    4. Delete password \n    5. Quit \nSet number");
        let mut num = String::new();
        for i in std::io::stdin().lines(){ num = i.unwrap(); break;}
        let rr = num.as_str();
        match rr {
            "2" => {
                let mut d = 0;
                for i in file::get_all_ps(format!("{}/.passs/", file::get_path_to_passs())){
                    let data = file::read_from(format!("{}/.passs/{i}.ps", file::get_path_to_passs()));
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
                let path = format!("{}/.passs/{}.ps", file::get_path_to_passs(), str).replace("\n", "");
                if file::check_file(path.clone()){
                    println!("Your old pass : {}", pass::from_vec_to_string(decrypt_thats_all(file::read_from(path.clone()), key.clone())).replace("\n", ""));
                    let pass = "0";
                    let encryptedpass = encrypt_thats_all(pass.as_bytes().to_vec(), key.clone());
                    file::rmfile(path.clone());
                    file::create_new_file(path.clone());
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
                let path = format!("{}/.passs/{}.ps", file::get_path_to_passs(), str).replace("\n", "");
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
fn check_pass(key : Vec<u8>) -> bool{
    let data = file::read_from(format!("{}/.passs/checkpass", file::get_path_to_passs()));
    let decinfo = decrypt_thats_all(data, key);
    let res = pass::from_vec_to_string(decinfo.to_vec());
    if res == "TRUE".to_string(){
        return true;
    }
    else {
        return false;
    }
}
fn write_pass(password : &String, key : Vec<u8>){
    let t = file::get_all_ps(format!("{}/.passs/", file::get_path_to_passs())).len();
    let newfilepath = format!("{}/.passs/{t}.ps", file::get_path_to_passs());
    file::create_new_file(newfilepath.clone());
    let encryptedpass = encrypt_thats_all(password.as_bytes().to_vec(), key.clone());
    file::write_into(aes256::concat_from_blocks_to_arr(encryptedpass), newfilepath);
}