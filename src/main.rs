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
        file::create_new("passs/0.ps".to_string());
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
            let pass = get_comapass();
            let encryptedpass = encrypt_thats_all(pass.as_bytes().to_vec(), key.clone());
            file::write_into(aes256::concat_from_blocks_to_arr(encryptedpass), "passs/0.ps".to_string());
            println!("Password has been write sucesfully");
        }
        else if rr == "2"{
            let data = file::read_from("passs/0.ps".to_string());
            println!("Readed data : {:?}", data);
            if data.len() == 0 || data.len() == 1{
                println!("You have no passwords. Use menu for writing new one\n");
                continue;
            }
            let n = decrypt_thats_all(data, key.clone());
            println!("Decrypted data : {:?}", n);
            for i in n{
                println!("{:?}", pass::from_vec_to_string(i).replace("\n", ""));
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
fn decrypt_thats_all(data : Vec<u8>, key : Vec<u8>) -> Vec<Vec<u8>>{
    let nn = aes256::spilt_into_bloks(data);
    println!("Splited into blocks : {:?}", nn);
    let mut nvec = vec![vec![0]];
    nvec.remove(0);
    for i in nn{
        let kk = aes256::decrypt_data(i.as_slice(), key.as_slice());
        nvec.push(kk);
    }
    let nig = aes256::concat_from_blocks_to_arr(nvec);
    let kk2 = pass::split_arr_into_passwords(nig.clone());
    let mut jr2 = vec![vec![0]];
    println!("Concated arr : {:?}", kk2);
    jr2.remove(0);
    for i in kk2{
        jr2.push(pass::change_pass_from_16_bytes_to_normal(i));
    }
    return jr2;
}
pub fn check_pass(key : Vec<u8>) -> bool{
    return true;
    
}