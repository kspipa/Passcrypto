
mod encrypt;
mod pass;
mod file;
fn main(){
    let t = "TRUE".as_bytes();
    let l = file::read_from("src/check".to_string());
}
fn start(){
    let mut pass = String::new();
    println!("Set your manager password : ");
    std::io::stdin().read_line(&mut pass).unwrap();
    let mm = pass.as_bytes();
    let key = pass::get_key_from_pass(mm);
    let binding = pass::change_pass("TRUE".as_bytes());
    println!("Not encrypted data : {:?}", binding);
    let encrypt_data_check_file = vec![encrypt::encrypt_data(binding.as_slice(), &key)];
    let sin = encrypt_data_check_file.clone();
    file::write_into(encrypt_data_check_file, "src/check.check".to_string());
    println!("{:?} : writed in file", sin[0]);
    get_comapass(key);
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
fn thaloop(){
    
}