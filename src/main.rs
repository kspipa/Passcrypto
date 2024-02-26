use cursive::Cursive;
use cursive::views::*;
use cursive::traits::*;
use file::*;
use pass::Passcryptopass;
mod aes256;
mod pass;
mod file;
mod server;
fn main(){
    let neg = std::env::args();
    let mut gsd = Vec::<String>::new();
    for i in neg{
        gsd.push(i);
    }
    if gsd.len() > 1{
        if gsd[1] == "-h".to_string(){
            println!("Passcrypto 1.0.0\nHow to use:\n   --server : turn in the server mode\n");
            return;
        }
        else if gsd[1] == "--server".to_string() {
            server::serverstart()
        }
    }
    let mut siv = cursive::default();
    start(&mut siv);
    siv.run();
}
fn get_pass(key : Vec<u8>, path : &str){
    let data = encrypt_thats_all("TRUE".as_bytes().to_vec(), key.clone());
    file::write_into(aes256::concat_from_blocks_to_arr(data), path.to_string());
}
fn get_hashes_from_decr_files(path: &str, key : Vec<u8>) -> Jsondb{
    let res = Jsondb::from(&pass::from_vec_to_string(decrypt_thats_all(file::read_from(path.clone().to_string()), key.clone())), key, path.clone().to_string());
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
fn check_pass(key : Vec<u8>, path : &str) -> bool{
    let data = &file::read_from(path.to_string())[0..16];
    let decinfo = decrypt_thats_all(data.to_vec(), key);
    let res = pass::from_vec_to_string(decinfo);
    if res == "TRUE".to_string(){
        return true;
    }
    else {
        return false;
    }
}
fn write_pass(db : &mut Jsondb, mut pass : Passcryptopass){
    let path = &db.positpath.clone();
    let filepath = db.filepath.clone();
    db.add_pass(path, pass.to_json());
    let encryptedpass = encrypt_thats_all(db.to_string().as_bytes().to_vec(), db.key.to_vec());
    file::rewrite(filepath, aes256::concat_from_blocks_to_arr(encryptedpass.clone()));
}
fn start(siv : &mut Cursive) {
    siv.add_layer(ResizedView::with_fixed_size((30, 10), Dialog::around(LinearLayout::vertical()
        .child(LinearLayout::horizontal().child(TextView::new("Path2db  : ")).child(EditView::new().with_name("path").fixed_size((15, 1))))
        .child(DummyView)
        .child(DummyView)
        .child( LinearLayout::horizontal().child(TextView::new("Password : ")).child(EditView::new().secret().with_name("password").fixed_size((15, 1))))
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(LinearLayout::horizontal().child(Button::new("Quit", Cursive::quit)).child(ResizedView::with_fixed_size((15, 0),DummyView)).child(Button::new("Ok", move |x| {
            let key = pass::get_hash_from_pass(x.call_on_name("password", |v : &mut EditView| {v.get_content()}).unwrap().as_bytes());
            let path = x.call_on_name("path", |s : &mut EditView|{return s.get_content().clone()}).unwrap().to_string();
            if !check_file(path.clone()){
                let mut db = newdb(path.clone(), key.clone());
                get_pass(key, &path);
                right(x, db);
            }
            else {
                if check_pass(key.clone(), &path){
                    let mut l = get_hashes_from_decr_files(&path,key.clone());
                    right(x, l);
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
fn right(ui : &mut Cursive, passs : Jsondb){
    ui.set_user_data(passs);
    ui.call_on_name("password", |b : &mut EditView| {b.set_content("")});
    let select = SelectView::<String>::new().on_submit( move |x: &mut Cursive , c : &str| {}).with_name("select").fixed_size((100, 5));
    let sh = Dialog::around(EditView::new().on_edit(|kk: &mut Cursive, path: &str, sixe: usize| {}).on_submit(|kk: &mut Cursive, path: &str| {}).with_name("edit").fixed_size((80, 1)));
    let menu = Dialog::around(SelectView::<String>::new().on_submit(|s: &mut Cursive, xsize: &String| {}).with_name("select1").fixed_size((80, 100)));
    let passtextarea = Dialog::around(LinearLayout::vertical()
        .child(LinearLayout::horizontal().child(TextView::new("Title :    ")).child(TextView::new("").with_name("title").fixed_size((80, 1))))
        .child(DummyView.fixed_size((1, 1)))
        .child(LinearLayout::horizontal().child(TextView::new("Username : ")).child(TextView::new("").with_name("username").fixed_size((80, 1))))
        .child(LinearLayout::horizontal().child(TextView::new("Password : ")).child(TextView::new("").with_name("password").fixed_size((80, 1))))
        .child(LinearLayout::horizontal().child(TextView::new("Url :      ")).child(TextView::new("").with_name("url").fixed_size((80, 1))))
        .child(LinearLayout::horizontal().child(TextView::new("Notes :    ")).child(TextView::new("").with_name("notes").fixed_size((80, 1))))).fixed_size((110,20));
    let dialog = Dialog::around(LinearLayout::vertical()
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Write new", move |g: &mut Cursive| {})))
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Change", move |g| {})))
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Delete", move |g| {delete_pass(g);})))
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Quit", |g| {g.pop_layer();})))).fixed_size((50, 100));
    let liner = LinearLayout::horizontal().child(LinearLayout::vertical().child(sh).child(menu)).child(passtextarea).child(dialog);
    ui.add_layer(liner);
    let passs = ui.user_data::<Jsondb>().unwrap();
    for i in passs.getall("/"){
        add_passwords_in_list(Passcryptopass::from_json(i), ui);
    }
}
fn delete_pass(s: &mut Cursive){
    let mut passs = s.with_user_data(|hash :  &mut Jsondb| {return hash.clone();}).unwrap();
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    let _ii = select.selection();
    let mut _num : i8 = 120;
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No pass to remove")),
        Some(focus) => {
            select.remove_item(focus);
            passs.deletebypath(&format!("{}/{}", passs.positpath, _ii.unwrap()));
            s.set_user_data(passs);
        }
    }
}
fn get_compass(ui : &mut Cursive, data : Option<Passcryptopass>){
    ui.add_layer(ResizedView::with_fixed_size((50, 10), Dialog::around(LinearLayout::vertical()
        .child(LinearLayout::horizontal().child(TextView::new("Title")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().with_name("edittitle").fixed_size((20 , 2))))
        .child(DummyView.fixed_size((1, 1)))
        .child(LinearLayout::horizontal().child(TextView::new("Username")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().with_name("editusername").fixed_size((20 , 2))))
        .child(LinearLayout::horizontal().child(TextView::new("Password")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().with_name("editpasss").fixed_size((20 , 2))).child(DummyView).child(Button::new("X", |z| {z.call_on_name("passs", |f : &mut EditView| {f.set_content(pass::generate_password(25))});})))
        .child(LinearLayout::horizontal().child(TextView::new("Url")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().with_name("editurl").fixed_size((20 , 2))))
        .child(LinearLayout::horizontal().child(TextView::new("Notes")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().with_name("editnotes").fixed_size((20 , 2))))
        .child(DummyView)
        .child(LinearLayout::horizontal().child(Button::new("Quit", |v| {v.pop_layer();})).child(DummyView.fixed_size((15, 1))).child(Button::new("Copy", |c| {let all_String = get_info(c);pass::copy_to_clipboard(all_String.clone().get_password().clone());c.add_layer(Dialog::info("Password has been copied"));})).child(DummyView.fixed_size((15, 1))).child(Button::new("Ok", move |x: &mut Cursive| {
    let all_String = get_info(x);
    let mut hass = x.user_data::<Jsondb>().unwrap().clone();
    match check_hashmap(&mut hass, all_String.clone()) {
        Ok(_) => 1,
        Err(_) => {x.add_layer(Dialog::info("This password already writen"));return;}
    };
    write_pass(&mut hass, all_String.clone());
    x.set_user_data(hass);
    add_passwords_in_list(all_String, x);
    x.pop_layer();
    }).with_name("Ok_button"))))));
    if data.is_some(){
        set_info(ui, data.unwrap());
        ui.call_on_name("Ok_button", move |c : &mut Button| {
            c.set_callback(move |x|{
                let all_string = get_info(x);
                let mut hass = x.user_data::<Jsondb>().unwrap().clone();
                match check_hashmap(&mut hass, all_string.clone()){
                    Ok(_) => 1,
                    Err(_) => {x.pop_layer(); return;}
                };
                let _num = delete_pass(x);
                let newpaass = write_pass(&mut hass,all_string.clone());
                x.set_user_data(newpaass);
                add_passwords_in_list(all_string, x);
                x.pop_layer();
            });
        });
    }
}
fn add_passwords_in_list(mut passs : Passcryptopass, ui : &mut Cursive){
    ui.call_on_name("select", |x : &mut SelectView| {x.add_all_str(vec![passs.get_title()])});
}
fn check_hashmap(passs : &mut Jsondb, mut pass : Passcryptopass) -> Result<bool , bool>{
    for i in passs.get_passs(None){
        if Passcryptopass::from_json(i).get_title() == pass.get_title(){
            return Err(false);
        }
    }
    return Ok(true);
}
fn get_info(x : &mut Cursive) -> Passcryptopass{
    let edittitle = x.call_on_name("edittitle", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let editusername = x.call_on_name("editusername", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let editpasss = x.call_on_name("editpasss", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let editurl = x.call_on_name("editurl", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let editnotes = x.call_on_name("editnotes", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    return Passcryptopass::from_vec(vec![edittitle, editusername, editpasss, editurl, editnotes]);
}
fn set_info(x : &mut Cursive, mut password : Passcryptopass){
    x.call_on_name("edittitle", |b: &mut EditView| {b.set_content(password.get_title().clone())}).unwrap();
    x.call_on_name("editusername", |b: &mut EditView| {b.set_content(password.get_username().clone())}).unwrap();
    x.call_on_name("editpasss", |b: &mut EditView| {b.set_content(password.get_password().clone())}).unwrap();
    x.call_on_name("editurl", |b: &mut EditView| {b.set_content(password.get_url().clone())}).unwrap();
    x.call_on_name("editnotes", |b: &mut EditView| {b.set_content(password.get_notes().clone())}).unwrap();
}