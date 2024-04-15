use cursive::Cursive;
use cursive::views::*;
use cursive::traits::*;
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
mod libs;
use std::io::{Write, stdout};
use libs::*;
use libs::file::*;
use libs::pass::Passcryptopass;
mod server;
fn main(){
    let neg = std::env::args();
    let mut gsd = Vec::<String>::new();
    for i in neg{
        gsd.push(i);
    }
    if gsd.len() > 1{
        if gsd[1] == "-h".to_string(){
            println!("Passcrypto 1.0.0\nHow to use:\n   --server : turn in the server mode\n    --cli : use comandline like interface");
            return;
        }
        else if gsd[1] == "--server".to_string() {
            server::serverstart();
            return;
        }
        else if gsd[1] == "--cli" {
            clistart();
            return;
        }
        else if gsd[1] == "--clicon" {
            cliconstart();
            return;
        }
    }
    let mut siv = cursive::default();
    start(&mut siv);
    siv.run();
}
fn get_pass(key : Vec<u8>, path : &str , db : &mut Jsondb){
    let data = encrypt_thats_all(["TRUE".as_bytes().to_vec(), db.to_string().into_bytes()].concat(), key.clone());
    file::write_into(data, path.to_string());
}
fn get_hashes_from_decr_files(path: &str, key : Vec<u8>) -> Jsondb{
    let decrdata = &pass::from_vec_to_string(decrypt_thats_all(file::read_from(path.to_string()).to_vec(), key.clone()));
    let res = Jsondb::from(&decrdata[4..decrdata.len()],key.clone(), path.to_string());
    return res;
}
fn encrypt_thats_all(data : Vec<u8>, key : Vec<u8>) -> Vec<u8> {
    let binding = pass::pad(data.as_slice());
    let newstr = aes256::spilt_into_bloks(binding);
    let mut nvec:Vec<Vec<u8>> = vec![vec![0]];
    nvec.remove(0);
    for i in newstr{
        nvec.push(aes256::encrypt_data(i.as_slice(), key.as_slice()));
    }
    return aes256::concat_from_blocks_to_arr(nvec);
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
    let data = &file::read_from(path.to_string());
    let decinfo = decrypt_thats_all(data.to_vec(), key);
    let res = &pass::from_vec_to_string(decinfo[0..4].to_vec());
    if res.contains("TRUE"){
        return true;
    }
    else {
        return false;
    }
}
fn getstring(out : &str) -> String{
    let mut buf = String::new();
    print!("{}", out);
    let _ = stdout().flush();
    for i in std::io::stdin().lines(){
        buf = i.unwrap().replace("\n", "");
        break;
    }
    return buf;
}
fn write(db : &mut Jsondb){
    let filepath = db.filepath.clone();
    let encryptedpass = encrypt_thats_all(["TRUE".as_bytes().to_vec(), db.to_string().as_bytes().to_vec()].concat(), db.key.to_vec());
    file::rewrite(filepath, encryptedpass.clone());
}
fn cliconstart(){
    let ip = getstring("Ip:port : ");
    let username = getstring("Username : ");
    let password = pass::getpass();
    let closekey = pass::fillwithrand();
    let newclosekey = [vec![200, 215, 188, 50, 67, 90], closekey.clone()].concat();
    let kk = pass::get_hash_from_pass([username.as_bytes(), password.as_bytes()].concat().as_mut_slice());
    let authdata = encrypt_thats_all(newclosekey, kk);
    client(&ip, &authdata, &closekey);

}
fn clistart(){
    let path = getstring("Path : ");
    let key = pass::get_hash_from_pass(pass::getpass().as_bytes());
    if !check_file(path.clone()){
        create_new_file(path.clone());
        let mut db = newdb(path.clone(), key.clone());
        get_pass(key, &path, &mut db);
        cliright(&mut db)
    }
    else {
        if check_pass(key.clone(), &path){
            let mut l = get_hashes_from_decr_files(&path,key.clone());
            cliright(&mut l);
        }
        else {
            println!("Wrong pass");
        }
    }
}
fn start(siv : &mut Cursive) {
    siv.add_layer(ResizedView::with_fixed_size((30, 10), Dialog::around(LinearLayout::vertical()
        .child(LinearLayout::horizontal().child(TextView::new("Path     : ")).child(EditView::new().with_name("path").fixed_size((15, 1))))
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
                create_new_file(path.clone());
                let mut db = newdb(path.clone(), key.clone());
                get_pass(key, &path, &mut db);
                right(x, db);
            }
            else {
                if check_pass(key.clone(), &path){
                    let l = get_hashes_from_decr_files(&path,key.clone());
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
fn seton(s : &mut Cursive, xsize: &String){
    if xsize.contains(".ps"){
        let l = s.with_user_data(|hash :  &mut Jsondb|{let res = format!("{}/{}", hash.positpath.clone(), xsize);let t = hash.get_pass(res.as_str()).unwrap();Passcryptopass::from_json(t.clone())}).unwrap();
        let path = s.with_user_data(|hash :  &mut Jsondb|{hash.positpath.clone()}).unwrap();
        s.call_on_name("edit", |s : &mut EditView|{s.set_content(format!("{}/{}", path, l.clone().get_title()))});
        set_info_to_list(s, l);
    }
    else {
        let path = s.with_user_data(|hash :  &mut Jsondb|{hash.positpath.clone()}).unwrap();
        s.call_on_name("edit", |s : &mut EditView|{s.set_content(format!("{}/{}", path, xsize))});
    }
}
fn cliright(db : &mut Jsondb){
    println!("Welcome to Passcrypto password database");
    println!("Print help to see comands");
    loop{
        let path = db.positpath.clone();
        let ans = getstring(format!("{} > ", if path == "".to_string(){"/"}else{&path}).as_str());
        match ans.as_str(){
            "ls" => {for i in db.getall(None).unwrap(){println!("{}",i["name"])}},
            "help" => println!("ls : get dirs and passes in dir\ncat <passname> : get info from password\nmkdir <name>: add new dir\ntouch <title> <username> <password> <url> <notes>: add new pass\n      There are also some special signs to indicate different commands:\n          <!> : if any cell is empty\n          <&<num>> : if you want get random letters in cell. <Num> is number of letters\n          if there is more than one word in a cell, put it in <>\n          Examples : \n            touch github kspipa &15 https://github.com/ <This is password from github!>\n            touch Gmail ! password123123 gmail.com !\ncd <dirname>: go to dir\npwd : get current location\nrm <name>: delete dir or pass\nch <name>: change data in dir or pass\ncp : copy password from pass to the clipboard\nexit : exit from database"),
            "pwd" => println!("{}",path),
            "exit" => break,
            "" => {},
            _ => {},
        }
        if ans.contains("mkdir"){
            let name = ans.replace("mkdir ", "");
            if check_hashmap(db, name.clone()).is_ok(){
                db.add_dir(if path == "/".to_string(){""}else{&path}, name.as_str());
                write(db);
                continue;
            }
            else{
                println!("This dir already writen");
                continue;
            }

        }
        if ans.contains("rm"){
            let name = ans.replace("rm ", "");
            db.deletebypath(&format!("{}/{}",path,name), name.contains(".ps"));
            write(db);
            continue;
        }
        if ans.contains("cd"){
            let name = ans.replace("cd ", "");
            if name == "/"{
                db.gotupath("");
            }
            if name.contains(".."){
                db.gotupath(&getpathwithoutps(path.clone(), 1));
            }
            if name.starts_with("/"){
                db.gotupath(&name);
            }
            else {
                db.gotupath(format!("{}/{}", path, name).as_str());
            }
            continue;
        }
        if ans.contains("cat"){
            let name = ans.replace("cat ", "");
            let mut newpath = format!("{}/{}", path, name);
            if name.starts_with("/"){
                newpath = name.clone();
            }
            match db.get_pass(&newpath){
                Some(t) => {let mut password = Passcryptopass::from_json(t.clone());
                    println!("\nTitle : {}\nUsername : {}\nPassword : **********\nUrl : {}\nNotes : {}", password.get_title(), password.get_username(), password.get_url(), password.get_notes());
                    match getstring("\nGet password? <y,n> : ").as_str(){
                        "y" => println!("Password : {}", password.get_password()),
                        _ => println!("Ok"),
                    }},
                None => {println!("Wrong path")},
            }
        }
        if ans.contains("touch"){
            let mut name = ans.replace("touch ", "");
            while name.find("&").is_some(){
                let t = name.find("&").unwrap();
                let mut q = String::new();
                let mut r = 1;
                loop{
                    let y = name.chars().nth(t+r);
                    if y.is_none() || &y.unwrap().to_string() == " "{
                        break;
                    }
                    q += &y.unwrap().to_string();
                    r += 1;
                }
                let delete = format!("&{}", q);
                name = name.replace(&delete, &pass::generate_password(q.parse::<usize>().unwrap()))
            }
            while name.find("<").is_some(){
                let t = name.find("<").unwrap();
                let mut q = String::new();
                let mut r = 1;
                loop{
                    let y = name.chars().nth(t+r).unwrap().to_string();
                    if &y == ">"{
                        name.remove(t+r);
                        break;
                    }
                    q += &y;
                    r += 1;
                }
                name.remove(t);
                name = name.replace(&q, &q.replace(" ", "!"));
            }
            if name.split(" ").count() == 5{
                let pass: Vec<String> = name.split_whitespace().map(|x|{x.to_string()}).collect();
                let mut newpass = Passcryptopass::from_vec(pass);
                let oldtitle = newpass.get_title().clone();
                if &oldtitle == "!"{
                    println!("Title required");
                    continue;
                }
                let mut newpasvec: Vec<String> = Vec::new();
                for i in newpass.to_vec(){
                    newpasvec.push(i.replace("!", " "))
                }
                newpass = Passcryptopass::from_vec(newpasvec);
                newpass.set_title(format!("{}.ps",oldtitle));
                if check_hashmap(db, format!("{}.ps",oldtitle)).is_ok(){
                    db.add_pass(&path, newpass.to_json());
                    write(db);
                    continue;
                }
                else {
                    println!("This pass already writen");
                    continue;
                }
            }
            else {
                println!("Invalid syntax");
            }
        }
    }
}
fn right(ui : &mut Cursive, passs : Jsondb){
    ui.set_user_data(passs);
    ui.call_on_name("password", |b : &mut EditView| {b.set_content("")});
    let sh = Dialog::around(EditView::new().on_submit(|kk: &mut Cursive, path: &str| 
        {
            if path.contains(".ps"){
                if path.contains(".."){
                    gotu(kk, &getpathwithoutps(path.to_string(), 3));
                    seton(kk, &"".to_string());
                    return;
                }
                let mut passs = kk.with_user_data(|hash :  &mut Jsondb| {return hash.clone();}).unwrap();
                let neg = match passs.gotupath(path){
                    Some(t) => t,
                    None => {kk.add_layer(Dialog::info("Wrong path"));return;}
                };
                gotu(kk, &getpathwithoutps(path.to_string(), 1));
                get_compass(kk, Some(Passcryptopass::from_json(neg.clone())), None)
            }
            else {
                if path.contains(".."){
                    gotu(kk, &getpathwithoutps(path.to_string(), 2));
                    seton(kk, &"".to_string());
                }
                else {
                    gotu(kk, path);
                }
            }
        }
    ).with_name("edit").fixed_size((80, 1)));
    let menu = SelectView::<String>::new().on_submit(|s: &mut Cursive, xsize: &String| {
        if !xsize.contains(".ps"){
            seton(s, xsize);
            let l = s.with_user_data(|hash :  &mut Jsondb| {return hash.positpath.clone();}).unwrap();
            gotu(s, format!("{}/{}", l, xsize).as_str());
        }
        else {
            seton(s, xsize);
        }
    }).on_select(|s: &mut Cursive, xsize: &String| {seton(s, xsize)}).with_name("select1").fixed_size((80, 100));
    let passtextarea = Dialog::around(LinearLayout::vertical()
        .child(LinearLayout::horizontal().child(TextView::new("Title    : ")).child(TextView::new("").with_name("title").fixed_size((80, 1))))
        .child(DummyView.fixed_size((1, 1)))
        .child(LinearLayout::horizontal().child(TextView::new("Username : ")).child(TextView::new("").with_name("username").fixed_size((80, 1))))
        .child(LinearLayout::horizontal().child(TextView::new("Password : ")).child(TextView::new("").with_name("password").fixed_size((80, 1))))
        .child(LinearLayout::horizontal().child(TextView::new("Url      : ")).child(TextView::new("").with_name("url").fixed_size((80, 1))))
        .child(LinearLayout::horizontal().child(TextView::new("Notes    : ")).child(TextView::new("").with_name("notes").fixed_size((80, 1))))).fixed_size((110,20));
    let dialog = Dialog::around(LinearLayout::vertical().child(Dialog::around(LinearLayout::vertical()
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Write new", move |g: &mut Cursive| {get_compass(g, None, None)})))
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Change", move |g| {
            if get_selected_name(g).contains(".ps"){
                let l = get_info_from_list(g);get_compass(g, Some(l), None)
            }
            else{
                let l = get_selected_name(g);get_compass(g, None, Some(l))
            }
        })))
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Delete", move |g| {let mut hass = g.user_data::<Jsondb>().unwrap().clone();delete(g, &mut hass);g.set_user_data(hass)})))
        .child(ResizedView::with_fixed_size((5, 2), Button::new("Quit", |g| {g.pop_layer();})))))).fixed_size((50, 100));
    let firstmenu = SelectView::<String>::new().on_submit(|s: &mut Cursive, xsize: &String| {
        let path = s.with_user_data(|hash :  &mut Jsondb| {return hash.positpath.clone();}).unwrap();
        if xsize.contains(".."){
            gotu(s, &getpathwithoutps(path.to_string(), 1));
            seton(s, &"".to_string());
        }
        else {
            s.call_on_name("edit", |a : &mut EditView|{a.set_content(path)});
        }
    }).with_name("firstmenu");
    let liner = LinearLayout::horizontal().child(LinearLayout::vertical().child(sh).child(Dialog::around(LinearLayout::vertical().child(firstmenu).child(menu)))).child(passtextarea).child(dialog);
    ui.add_fullscreen_layer(liner);
    ui.call_on_name("firstmenu", |l : &mut SelectView|{l.add_all_str(vec![".", ".."])});
    let passs = ui.user_data::<Jsondb>().unwrap();
    for i in passs.getall(Some("")).unwrap(){
        add_in_list(i["name"].to_string(), ui)
    }
}


fn gotu(s : &mut Cursive, path : &str){
    let mut passs = s.with_user_data(|hash :  &mut Jsondb| {return hash.clone();}).unwrap();
    let newhero = match passs.gotupath(path){
        Some(t) => t,
        None => {s.add_layer(Dialog::info("Path is not valid"));return;},
    };
    clear_list(s);
    for i in 0..newhero["dirs"].len(){
        add_in_list(newhero["dirs"][i]["name"].to_string(),s);
    }
    for i in 0..newhero["pass"].len(){
        add_in_list(newhero["pass"][i]["name"].to_string(),s);
    }
    s.set_user_data(passs);
}
fn delete(s: &mut Cursive, passs : &mut Jsondb){
    let mut select = s.find_name::<SelectView<String>>("select1").unwrap();
    let _ii = select.selection().unwrap();
    let bools = _ii.contains(".ps");
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No pass to remove")),
        Some(focus) => {
            select.remove_item(focus);
            passs.deletebypath(&format!("{}/{}", passs.positpath, _ii), bools);
            write(passs);
        }
    }
}
fn get_compass(ui : &mut Cursive, data : Option<Passcryptopass>, dirdata: Option<String>){
    let fulwindow = LinearLayout::vertical().child(LinearLayout::horizontal().child(Dialog::around(Button::new("Pass", |x|{uigetpass(x)}))).child(Dialog::around(Button::new("Dir", |x|{uigetdir(x)})))).with_name("getpass");
    ui.add_layer(ResizedView::with_fixed_size((55, 20), Dialog::around(fulwindow)));
    if data.is_some(){
        uigetpass(ui);
        set_info(ui, data.clone().unwrap());
        ui.call_on_name("Ok_button", move |c : &mut Button| {
            c.set_callback(move |x|{
                let mut all_string = get_info(x).clone();
                let mut hass = x.user_data::<Jsondb>().unwrap().clone();
                if all_string.get_title().clone() != data.clone().unwrap().get_title(){
                    match check_hashmap(&mut hass, all_string.clone().get_title()){
                        Ok(_) => 1,
                        Err(_) => {x.add_layer(Dialog::info("This pass already writen")); return;}
                    };
                }
                delete(x, &mut hass);
                hass.add_pass(&hass.positpath.clone(), all_string.clone().to_json());
                write(&mut hass);
                x.set_user_data(hass);
                add_in_list(all_string.get_title().clone(), x);
                x.pop_layer();
            });
        });
    }
    if dirdata.is_some(){
        uigetdir(ui);
        ui.call_on_name("dirname1", |b: &mut EditView| {b.set_content(dirdata.clone().unwrap())}).unwrap();
        ui.call_on_name("Ok_dir_button", move |c : &mut Button| {
            c.set_callback(move |x|{
                let all_string = x.call_on_name("dirname1", |b: &mut EditView| {b.get_content().to_string()}).unwrap().clone();
                let mut hass = x.user_data::<Jsondb>().unwrap().clone();
                if all_string.clone() != dirdata.clone().unwrap(){
                    match check_hashmap(&mut hass, all_string.clone()){
                        Ok(_) => 1,
                        Err(_) => {x.add_layer(Dialog::info("This dir already writen")); return;}
                    };
                }
                else {
                    return;
                }
                let fulpath = format!("{}/{}", hass.positpath, dirdata.clone().unwrap());
                hass.gotupath(&fulpath).unwrap()["name"] = all_string.clone().into();
                hass.gotupath(&getpathwithoutps(fulpath.clone(), 1));
                delete(x, &mut hass);
                x.set_user_data(hass);
                add_in_list(all_string.clone(), x);
                x.pop_layer();
            });
        });
    }
}
fn uigetpass(ui : &mut Cursive){
    let l = Dialog::around(LinearLayout::vertical()
    .child(LinearLayout::horizontal().child(TextView::new("Title    : ")).child(EditView::new().with_name("edittitle").fixed_size((30 , 2))).child(TextView::new(".ps")))
    .child(DummyView.fixed_size((1, 1)))
    .child(LinearLayout::horizontal().child(TextView::new("Username : ")).child(EditView::new().with_name("editusername").fixed_size((30 , 2))))
    .child(LinearLayout::horizontal().child(TextView::new("Password : ")).child(EditView::new().with_name("editpasss").fixed_size((30 , 2))).child(DummyView).child(Button::new("X", |z| {z.call_on_name("editpasss", |f : &mut EditView| {f.set_content(pass::generate_password(25))});})))
    .child(LinearLayout::horizontal().child(TextView::new("Url      : ")).child(EditView::new().with_name("editurl").fixed_size((30 , 2))))
    .child(LinearLayout::horizontal().child(TextView::new("Notes    : ")).child(EditView::new().with_name("editnotes").fixed_size((30 , 2))))
    .child(DummyView)
    .child(LinearLayout::horizontal().child(Button::new("Quit", |v| {v.pop_layer();})).child(DummyView.fixed_size((15, 1))).child(Button::new("Copy", |c| {let all_string = get_info(c);pass::copy_to_clipboard(all_string.clone().get_password().clone());c.add_layer(Dialog::info("Password has been copied"));})).child(DummyView.fixed_size((15, 1))).child(Button::new("Ok", move |x: &mut Cursive| {
    let mut all_string = get_info(x);
    let mut hass = x.user_data::<Jsondb>().unwrap().clone();
    match check_hashmap(&mut hass, all_string.clone().get_title()) {
        Ok(_) => 1,
        Err(_) => {x.add_layer(Dialog::info("This password already writen"));return;}
    };
    hass.add_pass(&hass.positpath.clone(), all_string.clone().to_json());
    write(&mut hass);
    x.set_user_data(hass);
    add_in_list(all_string.get_title().clone(), x);
    x.pop_layer();
    }).with_name("Ok_button"))));
    ui.call_on_name("getpass", |f : &mut LinearLayout|{f.remove_child(1);f.add_child(l)});
}
fn uigetdir(ui : &mut Cursive){
    ui.call_on_name("getpass", |t : &mut LinearLayout|{t.remove_child(1);t.add_child(
        Dialog::around(LinearLayout::vertical().child(LinearLayout::horizontal().child(TextView::new("Name : ")).child(EditView::new().with_name("dirname1").fixed_size((35 ,2)))).child(LinearLayout::horizontal().child(Button::new("Quit", |v| {v.pop_layer();})).child(DummyView.fixed_size((37, 1))).child(Button::new("Ok", |s|{
            let con = s.call_on_name("dirname1", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
            let mut hass = s.user_data::<Jsondb>().unwrap().clone();
            match check_hashmap(&mut hass, con.clone()){
                Ok(_) => 1,
                Err(_) => {s.add_layer(Dialog::info("This directory already writen"));return;}
            };
            hass.add_dir(&hass.positpath.clone(), con.as_str());
            write(&mut hass);
            add_in_list(con.to_string(), s);
            s.set_user_data(hass);
            s.pop_layer();
        }).with_name("Ok_dir_button")))));});
}
fn clear_list(ui : &mut Cursive){
    ui.call_on_name("select1", |x : &mut SelectView| {x.clear()});
}
fn add_in_list(dirs : String, ui : &mut Cursive){
    ui.call_on_name("select1", |x : &mut SelectView| {x.add_all_str(vec![dirs])});
}
fn check_hashmap(passs : &mut Jsondb, name: String) -> Result<bool , bool>{
    if passs.getall(None).unwrap().len() == 0{
        return Ok(true);
    }
    if name.contains(".ps") == false{
        for i in passs.get_dirs(None).unwrap(){
            if i["name"].to_string() == name{
                return Err(false);
            }
        }
    }
    else {
        for i in passs.get_passes(None).unwrap(){
            if Passcryptopass::from_json(i).get_title() == name{
                return Err(false);
            }
        }
    }
    
    return Ok(true);
}
fn get_info(x : &mut Cursive) -> Passcryptopass{
    let edittitle = x.call_on_name("edittitle", |b: &mut EditView| {format!("{}.ps", b.get_content())}).unwrap();
    let editusername = x.call_on_name("editusername", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let editpasss = x.call_on_name("editpasss", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let editurl = x.call_on_name("editurl", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let editnotes = x.call_on_name("editnotes", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    return Passcryptopass::from_vec(vec![edittitle, editusername, editpasss, editurl, editnotes]);
}
fn set_info(x : &mut Cursive, mut password : Passcryptopass){
    x.call_on_name("edittitle", |b: &mut EditView| {let mut ans = String::new();for i in password.get_title().clone().split(".ps"){ans = i.to_string();break;};b.set_content(ans)});
    x.call_on_name("editusername", |b: &mut EditView| {b.set_content(password.get_username().clone())});
    x.call_on_name("editpasss", |b: &mut EditView| {b.set_content(password.get_password().clone())});
    x.call_on_name("editurl", |b: &mut EditView| {b.set_content(password.get_url().clone())});
    x.call_on_name("editnotes", |b: &mut EditView| {b.set_content(password.get_notes().clone())});
}
fn get_info_from_list(x : &mut Cursive) -> Passcryptopass{
    let _ii = get_selected_name(x);
    let json = x.with_user_data(|t : &mut Jsondb|{t.gotupath(&format!("{}/{}", t.positpath.clone(), _ii)).unwrap().clone()}).unwrap();
    return Passcryptopass::from_json(json.clone());
}
fn get_selected_name(x: &mut Cursive) -> String{
    let select = x.find_name::<SelectView<String>>("select1").unwrap();
    return select.selection().unwrap().to_string();
}
fn set_info_to_list(x : &mut Cursive,mut password : Passcryptopass){
    x.call_on_name("title", |b: &mut TextView| {b.set_content(password.get_title().clone())}).unwrap();
    x.call_on_name("username", |b: &mut TextView| {b.set_content(password.get_username().clone())}).unwrap();
    x.call_on_name("password", |b: &mut TextView| {b.set_content(password.get_password().clone())}).unwrap();
    x.call_on_name("url", |b: &mut TextView| {b.set_content(password.get_url().clone())}).unwrap();
    x.call_on_name("notes", |b: &mut TextView| {b.set_content(password.get_notes().clone())}).unwrap();
}
fn client_auth(stream: &mut TcpStream, authdata : &[u8], newkey : &[u8]) -> u8{
    let mut buf = [0; 2048];
    let mut sixe = stream.read(&mut buf).unwrap();
    let gg = &buf[0..sixe];
    if gg != "200".as_bytes(){
        return 0;
    }
    println!("{:?}", newkey.len());
    stream.write(authdata);
    buf = [0; 2048];
    sixe = stream.read(&mut buf).unwrap();
    println!("{}", sixe);
    if decrypt_thats_all(buf[0..sixe].to_vec(), newkey.to_vec())[0] != 200{return 2;};
    println!("{:?}", newkey);
    return 1;
}
fn client(addr: &str, authdata : &[u8], newkey : &[u8]) {
    let server: SocketAddr = addr.parse().expect("Unable to parse socket address");
    let mut stream = TcpStream::connect(server).unwrap();
    match client_auth(&mut stream, authdata, newkey){
        2 => {println!("Problems with server");return;},
        0 => {println!("Problems with connect");return;},
        1 => {},
        _ => {return;}
    }
    loop{
        let req = getstring(">>");
        let mut buf = [0;16384];
        stream.write(encrypt_thats_all(req.as_bytes().to_vec(), newkey.to_vec()).as_mut_slice());
        let sixe = stream.read(&mut buf).unwrap();
        println!("{}", sixe);
        let decrmes = pass::from_vec_to_string(decrypt_thats_all(buf[0..sixe].to_vec(), newkey.to_vec()));
        println!("{}", decrmes);
        stream.flush();
    }
}