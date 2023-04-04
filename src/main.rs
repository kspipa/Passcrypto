mod encrypt;
mod pass;
fn main(){
    if std::fs::read_to_string("src/check").unwrap() == "TRUE"{
        start();
    }
}
fn start(){
    let mut pass = String::new();
    println!("Set your manager password : ");
    std::io::stdin().read_line(&mut pass).unwrap();
    let mm = pass.as_bytes();
    println!("norm pass : {:?}", mm);
    let encrypt = pass::change_pass(mm);
    let nnn = encrypt.as_slice();
    encrypt::encrypt_data(nnn, pass::change_pass("dasd".as_bytes()).as_slice());
}