use std::net;
use std::io::{Write, stdout, stdin};
mod pass;
mod file;

pub fn serverstart() {
    println!("Passcrypto server - server for password storage");
    println!("Enter 'help' for get all commands");
    let path = format!("{}/passcrypto", file::get_path_to_passs());
    if !file::check_dir(path.clone()){
        file::mkdir(path);
    }
    loop{
        start_console();
    }
}
fn start_console(){
    let mut buf = String::new();
    print!(">>");
    stdout().flush();
    for i in std::io::stdin().lines(){
        buf = i.unwrap().replace("\n", "");
        break;
    }
    treatment(&buf);
}
fn treatment(str : &str){
    match str{
        "setpass" => {},
        "help" => println!("Commands : \n    help : print this message \n    start : Start reciving messages \n    setpass : Set password for encrypt messages over the network \n    regnew : Register new account for password storage \n       'usual user' : have access only for him dir with passwords \n       'root' : have access for all dirs with passwords in the server \n    exit : exit from program \n    getusers : get all users"),
        "regnew" => {regnew()},
        "exit" => {},
        "getusers" => {for i in getusers(){println!("{}", i);}},
        _ => println!("Incorrect command : {}", str),
    }
}
fn regnew(){
    println!("Set username");
    let mut username = String::new();
    let path = format!("{}/passcrypto", file::get_path_to_passs());
    stdin().read_line(&mut username);
    if file::check_dir(format!("{}/{}", path, username.clone())){
        println!("This username is taken");
        regnew();
    }
    else {
        println!("Set password");
        let newpath = format!("{}/{}", path, username.clone());
        let pass = pass::getpass();
        file::mkdir(newpath.clone());
        let hash = pass::get_hash_from_pass(pass.as_bytes());
        let check = format!("{}/{}", newpath, "check_pass".to_string());
        file::create_new_file(check.clone());
        file::write_into(hash, check);
    }
}
fn getusers() -> Vec<String>{
    let path = format!("{}/passcrypto", file::get_path_to_passs());
    return file::check_files_in_dir(&path);
}
fn start(){
    
}