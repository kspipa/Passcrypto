
mod encrypt;
mod pass;
mod file;
fn main(){
}
fn start(){
    let key = get_first_pass();
}
fn get_first_pass() -> Vec<u8>{
    let mut pass = String::new();
    println!("Set your first master password");
    std::io::stdin().read_line(&mut pass).unwrap();
    let key = pass::get_key_from_pass(pass.as_bytes());
    file::create_new("passs/0.ps".to_string());
    return key;
}
fn get_comapass(nkey : Vec<u8>) -> Vec<Vec<u8>>{
    println!("Type your new pass");
    let mut pass = String::new();
    let mut res = String::new();
    std::io::stdin().read_line(&mut pass).unwrap();
    println!("Type resource");
    std::io::stdin().read_line(&mut res).unwrap();
    let l = format!("{}:{}", pass, res);
    let binding = pass::change_pass(l.as_bytes());
    let newstr = encrypt::spilt_into_bloks(binding.as_slice());
    let mut nvec:Vec<Vec<u8>> = vec![vec![0]];
    for i in newstr{
        nvec.push(encrypt::encrypt_data(i, nkey.as_slice()));
    }
    return nvec;
}
fn menu(){
    let key = get_first_pass();
    println!("Ok, what do you want to do : \n    1. Write new password \n    2. Quit \nSet number");
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).unwrap();
}