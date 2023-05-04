use cursive::Cursive;
use cursive::views::*;
use cursive::traits::*;
use std::collections::HashMap;
mod aes256;
mod pass;
mod file;
fn main(){
    let mut key = vec![0];
    let mut siv = cursive::default();
    if !file::check_file(format!("{}/.passs/checkpass", file::get_path_to_passs())){
        start(&mut siv, true);
    }
    else{
        if file::check_file(format!("{}/.passs/0.ps", file::get_path_to_passs())){
            for i in file::get_all_ps(format!("{}/.passs/", file::get_path_to_passs())){
                file::rmfile(format!("{}/.passs/{i}.ps", file::get_path_to_passs()));
            }
        }
        start(&mut siv, false);
    }
    siv.run();
}
fn get_pass(key : Vec<u8>){
    file::mkdir(format!("{}/.passs", file::get_path_to_passs()));
    let path = format!("{}/.passs/checkpass", file::get_path_to_passs());
    file::create_new_file(path.clone());
    let data = encrypt_thats_all("TRUE".as_bytes().to_vec(), key.clone());
    file::write_into(aes256::concat_from_blocks_to_arr(data), path);
}
fn change_pass(key : Vec<u8>){
    let mut str = String::new();
    println!("Print your password's number");
    std::io::stdin().read_line(&mut str).unwrap();
    let path = format!("{}/.passs/{}.ps", file::get_path_to_passs(), str).replace("\n", "");
    println!("Your old pass : {}", pass::from_vec_to_string(decrypt_thats_all(file::read_from(path.clone()), key.clone())).replace("\n", ""));
    let pass = "0";
    let encryptedpass = encrypt_thats_all(pass.as_bytes().to_vec(), key.clone());
    file::rmfile(path.clone());
    file::create_new_file(path.clone());
    file::write_into(aes256::concat_from_blocks_to_arr(encryptedpass), path);
    println!("Password has been change sucesfully");
}
fn delete_pass(){
    let mut str = String::new();
    println!("Print your password's number");
    std::io::stdin().read_line(&mut str).unwrap();
    let path = format!("{}/.passs/{}.ps", file::get_path_to_passs(), str).replace("\n", "");
    file::rmfile(path);
    println!("Your password succesfully deleted");
}
fn get_passwords_from_files(key : Vec<u8>) -> HashMap<String , i8>{
    let mut d = 0;
    let mut res : HashMap<String, i8> = HashMap::new();
    for i in file::get_all_ps(format!("{}/.passs/", file::get_path_to_passs())){
        let data = file::read_from(format!("{}/.passs/{i}.ps", file::get_path_to_passs()));
        let n = decrypt_thats_all(data, key.clone());
        res.insert(pass::from_vec_to_string(n).replace("\n", ""), d);
        d += 1
    }
    return res;
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
fn start(siv : &mut Cursive, first_or_not : bool) {
    let mut mes = String::new();
    if first_or_not{
        mes = "Print your password".to_string();
    }
    else {
        mes = "Set your password".to_string();
    }
    siv.add_layer(ResizedView::with_fixed_size((30, 10), Dialog::around(LinearLayout::vertical()
        .child(TextView::new(mes))
        .child(DummyView)
        .child(DummyView)
        .child(EditView::new().secret().with_name("password"))
        .child(DummyView)
        .child(LinearLayout::horizontal().child(Button::new("Quit", Cursive::quit)).child(ResizedView::with_fixed_size((15, 0),DummyView)).child(Button::new("Ok", move |x| {
            let key = pass::get_hash_from_pass(x.call_on_name("password", |v : &mut EditView| {v.get_content()}).unwrap().as_bytes());
            let mut res = false;
            let rr : HashMap<String, i8> = HashMap::new();
            if first_or_not{
                get_pass(key);
                right(x, rr);
            }
            else {
                if check_pass(key.clone()){
                    right(x, get_passwords_from_files(key));
                }
                else {
                    dont_right(x);
                }
            }
        }))))));
}

fn dont_right(ui : &mut Cursive){
    ui.add_layer(ResizedView::with_fixed_size((30, 10), Dialog::new().content(TextView::new("Your password is wrong")).button("Ok", |c| {c.pop_layer();})))
}
fn right(ui : &mut Cursive, passs : HashMap<String, i8>){
    ui.call_on_name("password", |b : &mut EditView| {b.set_content("")});
    let mut select = SelectView::<String>::new().on_submit( move |x , c : &str| {let pass = c.clone();get_compass(x, c, *passs.get(pass).unwrap())}).with_name("select").fixed_size((50, 5));
    let mut dialog = Dialog::around(LinearLayout::vertical()
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Write new", |g| {get_compass(g, "::" , -1);})))
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Delete", |g| {remove_password_from_list(0,g);})))
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Quit", |g| {g.pop_layer();})))).fixed_size((50, 100));
    ui.add_layer(Dialog::around(LinearLayout::horizontal().child(Dialog::around(select)).child(DummyView.fixed_size((125, 5))).child(dialog)).fixed_size((200, 100)));
}
fn get_compass(ui : &mut Cursive, pass : &str, id : i8){
    let mut ll = pass.split(":");
    ui.add_layer(ResizedView::with_fixed_size((50, 10), Dialog::around(LinearLayout::vertical()
        .child(LinearLayout::horizontal().child(TextView::new("Username")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().content(ll.next().unwrap()).with_name("name").fixed_size((20 , 2))))
        .child(LinearLayout::horizontal().child(TextView::new("Password")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().content(ll.next().unwrap()).with_name("passs").fixed_size((20 , 2))))
        .child(LinearLayout::horizontal().child(TextView::new("Source")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().content(ll.next().unwrap()).with_name("source").fixed_size((20 , 2))))
        .child(DummyView)
        .child(LinearLayout::horizontal().child(Button::new("Quit", |v| {v.pop_layer();})).child(DummyView.fixed_size((35, 1))).child(Button::new("Ok", move |x| {
    let username = x.call_on_name("name", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let password = x.call_on_name("passs", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let source = x.call_on_name("source", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    if id == -1{
        add_passwords_in_list(vec![format!("{}:{}:{}", username, password, source)], x);
    }
    else {
        remove_password_from_list(id as usize, x);
        add_passwords_in_list(vec![format!("{}:{}:{}", username, password, source)], x);
    }
    x.pop_layer();
    }))))));
}
fn add_passwords_in_list(passs : Vec<String>, ui : &mut Cursive){
    ui.call_on_name("select", |x : &mut SelectView| {x.add_all_str(passs)});
}
fn remove_password_from_list(id : usize, ui : &mut Cursive){
    ui.call_on_name("select", |x : &mut SelectView| {x.remove_item(id)});
}