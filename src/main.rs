
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
    let binding = pass::change_pass(&mm.to_vec());
    let mut d = Vec::from(["0".as_bytes()]); 
    d = encrypt::spilt_into_bloks(binding.as_slice());
    println!("split into blocks : {:?}", d);
    let biding = pass::get_key_from_pass("12345reqewqewq".as_bytes());
    let key = biding.as_slice();
    for i in d{
        encrypt::encrypt_data(i, key);
    }
}