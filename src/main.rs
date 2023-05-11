use cursive::Cursive;
use cursive::views::*;
use cursive::traits::*;
use std::collections::HashMap;
mod aes256;
mod pass;
mod file;
fn main(){
    let mut siv = cursive::default();
    if file::check_file(format!("{}/log",file::get_path_to_passs())){
        file::rmfile(format!("{}/log", file::get_path_to_passs()));
    }
    file::create_new_file(format!("{}/log", file::get_path_to_passs()));
    file::write_to_log("Start");
    if !file::check_file(format!("{}/checkpass", file::get_path_to_passs())){
        start(&mut siv, true);
    }
    else{
        start(&mut siv, false);
    }
    siv.run();
}
fn get_pass(key : Vec<u8>){
    file::mkdir(file::get_path_to_passs());
    let path = format!("{}/checkpass", file::get_path_to_passs());
    file::create_new_file(path.clone());
    let data = encrypt_thats_all("TRUE".as_bytes().to_vec(), key.clone());
    file::write_into(aes256::concat_from_blocks_to_arr(data), path);
}
fn get_passwords_from_files(key : Vec<u8>) -> HashMap<String , i8>{
    let mut res : HashMap<String, i8> = HashMap::new();
    for i in file::get_all_ps(file::get_path_to_passs()){
        let data = file::read_from(format!("{}/{i}.ps", file::get_path_to_passs()));
        res.insert(pass::from_vec_to_string(decrypt_thats_all(data, key.clone())).replace("\n", ""), i as i8);
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
    let data = file::read_from(format!("{}/checkpass", file::get_path_to_passs()));
    let decinfo = decrypt_thats_all(data, key);
    let res = pass::from_vec_to_string(decinfo.to_vec());
    if res == "TRUE".to_string(){
        return true;
    }
    else {
        return false;
    }
}
fn write_pass(password : &String, key : Vec<u8>, mut passs : HashMap<String, i8>) -> HashMap<String, i8>{
    let t = match file::get_all_ps(file::get_path_to_passs()).last(){
        None => 0,
        Some(h) => *h + 1
    };
    let newfilepath = format!("{}/{}.ps", file::get_path_to_passs(), t.clone());
    file::create_new_file(newfilepath.clone());
    let encryptedpass = encrypt_thats_all(password.as_bytes().to_vec(), key.clone());
    file::write_into(aes256::concat_from_blocks_to_arr(encryptedpass.clone()), newfilepath);
    passs.insert(pass::from_vec_to_string(aes256::concat_from_blocks_to_arr(encryptedpass)), t as i8);
    return passs;
}
fn start(siv : &mut Cursive, first_or_not : bool) {
    let mut mes = String::new();
    if first_or_not{
        mes = "Set your password".to_string();
        file::write_to_log("First time");
    }
    else {
        mes = "Print your password".to_string();
        file::write_to_log("Not first time");
    }
    siv.add_layer(ResizedView::with_fixed_size((30, 10), Dialog::around(LinearLayout::vertical()
        .child(TextView::new(mes))
        .child(DummyView)
        .child(DummyView)
        .child(EditView::new().secret().with_name("password"))
        .child(DummyView)
        .child(LinearLayout::horizontal().child(Button::new("Quit", Cursive::quit)).child(ResizedView::with_fixed_size((15, 0),DummyView)).child(Button::new("Ok", move |x| {
            let key = pass::get_hash_from_pass(x.call_on_name("password", |v : &mut EditView| {v.get_content()}).unwrap().as_bytes());
            let mut rr : HashMap<String, i8> = HashMap::new();
            if first_or_not{
                get_pass(key.clone());
                right(x, &mut rr, key);
            }
            else {
                if check_pass(key.clone()){
                    right(x, &mut get_passwords_from_files(key.clone()), key);
                }
                else {
                    dont_right(x);
                }
            }
        }))))));
}

fn dont_right(ui : &mut Cursive){
    file::write_to_log("Password not right");
    ui.add_layer(Dialog::info("Your password is wrong"));
}
fn right(ui : &mut Cursive, passs : &mut HashMap<String, i8>, key : Vec<u8>){
    file::write_to_log("Password is right");
    file::write_to_log(format!("passs Hashmap : {:?}", passs.clone()).as_str());
    ui.set_user_data(passs.clone());
    ui.call_on_name("password", |b : &mut EditView| {b.set_content("")});
    let _jj = key.clone();
    let hh = key.clone();
    let newkey = key.clone();
    let newpasss = passs.clone();
    let _ll = passs.clone();
    let select = SelectView::<String>::new().on_submit( move |x: &mut Cursive , c : &str| {get_compass(x, _jj.clone())}).with_name("select").fixed_size((50, 5));
    let dialog = Dialog::around(LinearLayout::vertical()
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Write new", move |g: &mut Cursive| {get_compass(g, hh.clone());})))
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Delete", move |g| {delete_pass(g, &mut _ll.clone())})))
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Quit", |g| {g.pop_layer();})))).fixed_size((50, 100));
    ui.add_layer(Dialog::around(LinearLayout::horizontal().child(Dialog::around(select)).child(DummyView.fixed_size((125, 5))).child(dialog)).fixed_size((200, 100)));
    for i in newpasss{
        add_passwords_in_list(i.0, ui);
    }
}
fn delete_pass(s: &mut Cursive, passs : &mut HashMap<String, i8>) {
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    let _ii = select.selection();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No pass to remove")),
        Some(focus) => {
            select.remove_item(focus);
            let _num = match passs.get(_ii.clone().unwrap().as_str()){
                Some(l) => {s.add_layer(Dialog::info(format!("find id : {l}"))); *l},
                None => {s.add_layer(Dialog::info(format!("Not find id : {:?}", passs.clone().into_values()))); 120}
            };
            if _num == 120{
                return;
            }
            let path = format!("{}/{}.ps", file::get_path_to_passs(), _num);
            file::rmfile(path);
            passs.remove(_ii.unwrap().as_str()).unwrap();
        }
    }
}
fn get_compass(ui : &mut Cursive, key : Vec<u8>){
    ui.add_layer(ResizedView::with_fixed_size((50, 10), Dialog::around(LinearLayout::vertical()
        .child(LinearLayout::horizontal().child(TextView::new("Username")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().with_name("name").fixed_size((20 , 2))))
        .child(LinearLayout::horizontal().child(TextView::new("Password")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().with_name("passs").fixed_size((20 , 2))))
        .child(LinearLayout::horizontal().child(TextView::new("Source")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().with_name("source").fixed_size((20 , 2))))
        .child(DummyView)
        .child(LinearLayout::horizontal().child(Button::new("Quit", |v| {v.pop_layer();})).child(DummyView.fixed_size((35, 1))).child(Button::new("Ok", move |x: &mut Cursive| {
    let username = x.call_on_name("name", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let password = x.call_on_name("passs", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let source = x.call_on_name("source", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let newpaass = write_pass(&format!("{}:{}:{}", username, password, source), key.clone(), x.with_user_data(|hash :  &mut HashMap<String, i8>| {return hash.clone();}).unwrap());
    x.set_user_data(newpaass);
    add_passwords_in_list(format!("{}:{}:{}", username, password, source), x);
    x.pop_layer();
    }))))));
}
fn add_passwords_in_list(passs : String, ui : &mut Cursive){
    ui.call_on_name("select", |x : &mut SelectView| {x.add_all_str(vec![passs])});
}