mod encrypt;
mod pass;
use sha2::{Digest , Sha256};
fn main(){
    if std::fs::read_to_string("src/check").unwrap() == "TRUE"{
        start();
    }
}
fn start(){
    let mut pass = String::new();
    println!("Set your manager password : ");
    std::io::stdin().read_line(&mut pass).unwrap();
    println!("pass as bytes : {:?}", pass.as_bytes());
    let changed = pass::change_pass(pass.as_bytes());
    println!("your new pass : {:?}", changed);
    println!("pass len : {}", changed.len())
}