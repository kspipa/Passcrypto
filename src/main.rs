use cursive::Cursive;
use cursive::views::*;
use cursive::traits::*;
use kpdb::Database;
use std::collections::HashMap;
mod aes256;
mod pass;
mod db;
mod ui;
fn main(){
    let mut siv = cursive::default();
    if !db::check_dir(db::get_path_to_passs()){
        db::mkdir(db::get_path_to_passs());
        start(&mut siv, true);
    }
    else{
        start(&mut siv, false);
    }
    siv.run();
}
fn get_passwords_from_files() -> HashMap<Vec<u8> , i8>{
    let mut res : HashMap<Vec<u8>, i8> = HashMap::new();
    for i in db::get_all_ps(db::get_path_to_passs()){
        let data = db::read_from(format!("{}/{i}.ps", db::get_path_to_passs()));
        res.insert(data, i as i8);
    }
    return res;
}
fn get_hashes_from_decr_files(key : Vec<u8>) -> HashMap<Vec<u8> , i8>{
    let mut res : HashMap<Vec<u8>, i8> = HashMap::new();
    for i in db::get_all_ps(db::get_path_to_passs()){
        let data = db::read_from(format!("{}/{i}.ps", db::get_path_to_passs()));
        res.insert(pass::get_hash_from_pass(decrypt_thats_all(data, key.clone()).as_mut_slice()), i as i8);
    }
    return res;
}
fn encrypt_thats_all(data : Vec<u8>, key : Vec<u8>) -> Vec<Vec<u8>> {
    let binding = pass::pad(data.as_slice());
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
    let yy = pass::unpad(jj);
    return yy;
}
fn check_pass(key : String, name: &str) -> bool{
    match db::open_db(name, key){
        Ok(_) => true,
        Err(_) => false
    }
}
fn write_pass(password : &String, key : Vec<u8>, mut passs : HashMap<Vec<u8>, i8>, num : u8) -> HashMap<Vec<u8>, i8>{
    let hash = pass::get_hash_from_pass(password.as_bytes());
    let mut t = match db::get_all_ps(db::get_path_to_passs()).last(){
        None => 0,
        Some(h) => *h + 1
    };
    if num == 120{}
    else {t = num;}
    let newfilepath = format!("{}/{}.ps", db::get_path_to_passs(), t.clone());
    db::create_new_file(newfilepath.clone());
    let encryptedpass = encrypt_thats_all(password.as_bytes().to_vec(), key.clone());
    db::write_into(aes256::concat_from_blocks_to_arr(encryptedpass.clone()), newfilepath);
    passs.insert(hash, t as i8);
    return passs;
}
fn start(siv : &mut Cursive, first_or_not : bool) {
    let mut mes = String::new();
    if first_or_not{
        mes = "Set your password".to_string();
    }
    else {
        mes = "Print your password".to_string();
    }
    siv.add_layer(ResizedView::with_fixed_size((30, 10), Dialog::around(LinearLayout::vertical()
        .child(TextView::new(mes))
        .child(DummyView)
        .child(DummyView)
        .child( EditView::new().secret().with_name("password"))
        .child(DummyView)
        .child(LinearLayout::horizontal().child(Button::new("Quit", Cursive::quit)).child(ResizedView::with_fixed_size((15, 0),DummyView)).child(Button::new("Ok", move |x| {
            let key = pass::to_hex(pass::get_hash_from_pass(x.call_on_name("password", |v : &mut EditView| {v.get_content()}).unwrap().as_bytes()).as_slice());
            if first_or_not{
                let mut database = db::newdb("Passcrypto", key);
                right(x, &mut database);
            }
            else {
                if check_pass(key.clone(), "Passcrypto"){
                    let mut database = db::open_db("Passcrypto", key).unwrap();
                    right(x, &mut database);
                }
                else {
                    dont_right(x);
                }
            }
        }))))));
}

fn dont_right(ui : &mut Cursive){
    ui.add_layer(Dialog::info("Your password is wrong"));
}
fn right(ui : &mut Cursive, database : &mut Database){
    ui.set_user_data(database.clone());
    ui.call_on_name("password", |b : &mut EditView| {b.set_content("")});
    let select = SelectView::<String>::new().on_submit( move |x: &mut Cursive , c : &str| {get_compass(x, _jj.clone(), c)}).with_name("select").fixed_size((100, 5));
    let dialog = Dialog::around(LinearLayout::vertical()
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Write new", move |g: &mut Cursive| {get_compass(g, hh.clone(), "");})))
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Change", move |g| {let info = g.call_on_name("select", |v : &mut SelectView| {return v.selection().unwrap();}).unwrap();get_compass(g, hj.clone(), info.as_str());})))
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Delete", move |g| {delete_pass(g);})))
        .child(DummyView.fixed_size((5, 38)))
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Quit", |g| {g.pop_layer();})))).fixed_size((50, 100));
    ui.add_layer(Dialog::around(LinearLayout::horizontal().child(Dialog::around(select)).child(DummyView.fixed_size((75, 5))).child(dialog)).fixed_size((200, 100)));
    for i in get_passwords_from_files(){
        add_passwords_in_list(pass::from_vec_to_string(decrypt_thats_all(i.0, newkey.clone())), ui);
    }
}
fn delete_pass(s: &mut Cursive) -> i8 {
    let mut passs = s.with_user_data(|hash :  &mut HashMap<Vec<u8>, i8>| {return hash.clone();}).unwrap();
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    let _ii = select.selection();
    let mut _num : i8 = 120;
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No pass to remove")),
        Some(focus) => {
            select.remove_item(focus);
            let encryptedpass = pass::get_hash_from_pass(_ii.clone().unwrap().as_bytes());
            _num = match passs.get(&((encryptedpass))){
                Some(l) => {*l},
                None => {120}
            };
            passs.remove(&(encryptedpass));
            s.set_user_data(passs);
            if _num == 120{
                return 120;
            }
            let path = format!("{}/{}.ps", db::get_path_to_passs(), _num);
            db::rmfile(path);
            return _num;
        }
    }
    return _num;
}
fn get_compass(ui : &mut Cursive, key : Vec<u8>, data : &str){
    let key2 = key.to_owned();
    ui.add_layer(ResizedView::with_fixed_size((50, 10), Dialog::around(LinearLayout::vertical()
        .child(LinearLayout::horizontal().child(TextView::new("Username")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().with_name("name").fixed_size((20 , 2))))
        .child(LinearLayout::horizontal().child(TextView::new("Password")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().with_name("passs").fixed_size((20 , 2))).child(DummyView).child(Button::new("X", |z| {z.call_on_name("passs", |f : &mut EditView| {f.set_content(pass::generate_password(25))});})))
        .child(LinearLayout::horizontal().child(TextView::new("Source")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().with_name("source").fixed_size((20 , 2))))
        .child(DummyView)
        .child(LinearLayout::horizontal().child(Button::new("Quit", |v| {v.pop_layer();})).child(DummyView.fixed_size((15, 1))).child(Button::new("Copy", |c| {let all_String = get_info(c);pass::copy_to_clipboard(all_String.clone());c.add_layer(Dialog::info(format!("Password : '{}' has been copied", all_String).as_str()));})).child(DummyView.fixed_size((15, 1))).child(Button::new("Ok", move |x: &mut Cursive| {
    let all_String = get_info(x);
    let hass = x.with_user_data(|hash :  &mut HashMap<Vec<u8>, i8>| {return hash.clone();}).unwrap();
    match check_hashmap(hass.clone(), all_String.clone()) {
        Ok(_) => 1,
        Err(_) => {x.add_layer(Dialog::info("This password already writen"));return;}
    };
    let newpaass = write_pass(&all_String, key.clone(), hass, 120);
    x.set_user_data(newpaass);
    add_passwords_in_list(all_String, x);
    x.pop_layer();
    }).with_name("Ok_button"))))));
    if data == ""{}
    else {
        set_info(ui, data);
        ui.call_on_name("Ok_button", move |c : &mut Button| {
            c.set_callback(move |x|{
                let all_string = get_info(x);
                let hass = x.with_user_data(|hash :  &mut HashMap<Vec<u8>, i8>| {return hash.clone();}).unwrap();
                match check_hashmap(hass.clone(), all_string.clone()){
                    Ok(_) => 1,
                    Err(_) => {x.pop_layer(); return;}
                };
                let _num = delete_pass(x);
                let newpaass = write_pass(&all_string, key2.clone(), hass, _num as u8);
                x.set_user_data(newpaass);
                add_passwords_in_list(all_string, x);
                x.pop_layer();
            });
        });
    }
}
fn add_passwords_in_list(passs : String, ui : &mut Cursive){
    ui.call_on_name("select", |x : &mut SelectView| {x.add_all_str(vec![passs])});
}
fn check_hashmap(passs : HashMap<Vec<u8>, i8>, string : String) -> Result<bool , bool>{
    let hash = pass::get_hash_from_pass(string.as_bytes());
    for i in passs{
        if i.0 == hash{
            return Err(false);
        }
    }
    return Ok(true);
}
fn get_info(x : &mut Cursive) -> String{
    let username = x.call_on_name("name", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let password = x.call_on_name("passs", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let source = x.call_on_name("source", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    return format!("{}:{}:{}", username, password, source);
}
fn set_info(x : &mut Cursive, info : &str){
    let splited = info.split(":");
    let mut num: Vec<&str> = Vec::new();
    for i in splited{
        num.push(i);
    }
    x.call_on_name("name", |b: &mut EditView| {b.set_content(num[0])});
    x.call_on_name("passs", |b: &mut EditView| {b.set_content(num[1])});
    x.call_on_name("source", |b: &mut EditView| {b.set_content(num[2])});
}