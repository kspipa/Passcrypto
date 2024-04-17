use std::io::{stdin, stdout, Read, Write};
#[path = "libs/mod.rs"] mod thing;
use thing::*;
use thing::file::*;
use thing::pass::Passcryptopass;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
pub fn serverstart() {
    clistart();
}

fn clistart(){
    let path = getstring("Path : ");
    let key = pass::get_hash_from_pass(pass::getpass().as_bytes());
    if !check_file(path.clone()){
        create_new_file(path.clone());
        let mut db = newdb(path.clone(), key.clone());
        get_pass(key, &path, &mut db);
        println!("Passcrypto server - server for password storage");
        println!("Enter 'help' for get all commands");
        treatment(&mut db);
    }
    else {
        if check_pass(key.clone(), &path){
            let mut l = get_hashes_from_decr_files(&path,key.clone());
            println!("Passcrypto server - server for password storage");
            println!("Enter 'help' for get all commands");
            treatment(&mut l);
        }
        else {
            println!("Wrong pass");
        }
    }
}
fn get_pass(key : Vec<u8>, path : &str , db : &mut Jsondb){
    let data = encrypt_thats_all(["TRUE".as_bytes().to_vec(), db.to_string().into_bytes()].concat(), key.clone());
    file::write_into(data, path.to_string());
}
fn get_hashes_from_decr_files(path: &str, key : Vec<u8>) -> Jsondb{
    let decrdata = &pass::from_vec_to_string(decrypt_thats_all(file::read_from(path.to_string()).to_vec(), key.clone(), true));
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
fn decrypt_thats_all(data : Vec<u8>, key : Vec<u8>, padding : bool) -> Vec<u8>{
    let mm = aes256::spilt_into_bloks(data);
    let mut newvec = vec![vec![0 as u8]];
    newvec.remove(0);
    for i in mm{
        newvec.push(aes256::decrypt_data(i.as_slice(), key.as_slice()));
    }
    let jj = aes256::concat_from_blocks_to_arr(newvec);
    let mut yy = Vec::<u8>::new();
    if padding{yy = pass::unpad(jj);}else {
        yy = jj;
    }
    return yy;
}
fn check_pass(key : Vec<u8>, path : &str) -> bool{
    let data = &file::read_from(path.to_string());
    let decinfo = decrypt_thats_all(data.to_vec(), key, true);
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
fn treatment(jsaon : &mut Jsondb){
    loop {
        let path = jsaon.positpath.clone();
        let strf = getstring(">>");
        let str = strf.as_str();
        if str.contains("deluser"){
            let name = str.replace("deluser ", "");
            let mut t = 0;
            let mut newstr = (String::new(), String::new(),String::new());
            for i in jsaon.getusers(){
                if i.0 == name{
                    newstr = i;
                    t += 1;
                    break;
                }
            }
            if t == 0{
                println!("This user not exist");
                continue;
            }
            jsaon.deleteuser(newstr);
            write(jsaon);
            continue;
        }
        if str.contains("cd"){
            let name = str.replace("cd ", "");
            if name == "/"{
                jsaon.gotupath("");
            }
            if name.contains(".."){
                jsaon.gotupath(&getpathwithoutps(path.clone(), 1));
            }
            if name.starts_with("/"){
                jsaon.gotupath(&name);
            }
            else {
                jsaon.gotupath(format!("{}/{}", path, name).as_str());
            }
            continue;
        }
        if str.contains("cat"){
            let name = str.replace("cat ", "");
            let mut newpath = format!("{}/{}", path, name);
            if name.starts_with("/"){
                newpath = name.clone();
            }
            match jsaon.get_pass(&newpath){
                Some(t) => {let mut password = Passcryptopass::from_json(t.clone());
                    println!("\nTitle : {}\nUsername : {}\nPassword : **********\nUrl : {}\nNotes : {}", password.get_title(), password.get_username(), password.get_url(), password.get_notes());
                    match getstring("\nGet password? <y,n> : ").as_str(){
                        "y" => println!("Password : {}", password.get_password()),
                        _ => println!("Ok"),
                    }},
                None => {println!("Wrong path");continue;},
            }
            continue;
        }
        if str.contains("getuser") && str != "getusers"{
            let name = str.replace("getuser ", "");
            let mut t = 0;
            for i in jsaon.getusers(){
                if i.0 == name{
                    println!("Username : {}\nPassword : {}\nEmail : {}", i.0, i.1, i.2);
                    t += 1;
                    break;
                }
            }
            if t != 0{
                continue;
            }
            println!("This user is not exists");
            continue;
        }
        if str.contains("setperm"){
            let name = str.replace("setperm ", "");
            let data : Vec<String> = name.split_whitespace().map(|t|{t.to_string()}).collect();
            if data.len() != 3{
                println!("Wrong syntax");
                continue;
            }
            let mut pathu = "";
            let pp = data[2].clone();
            let tt = &format!("{}/{}", path, pp.clone()).clone();
            if data[2] == "/".to_string(){
                pathu = "";
            }
            else if data[2].starts_with("/"){
                pathu = pp.as_str();
            }
            else {
                pathu = tt;
            }
            let mut perms = Vec::<String>::new();
            let mut users = Vec::<String>::new();
            for i in jsaon.getusers(){
                users.push(i.0.clone());
                perms.push(jsaon.get_perm(pathu, i.0));
            }
            jsaon.clear_perm(pathu);
            let mut t = 0;
            for i in users{
                if i == data[0]{
                    jsaon.set_perm(pathu, i, data[1].as_str());
                    t += 1;
                    continue;
                }
                jsaon.set_perm(pathu, i, if perms[t] == String::from(""){"--"}else{perms[t].as_str()});
                t += 1;
            }
            write(jsaon);
            jsaon.gotupath(&path);
            continue;
        }
        if str.contains("start"){
            let name = str.replace("start ", "");
            netserver(&format!("0.0.0.0:{}", name), jsaon);
        }
        match str{
            "help" => println!("Commands : \n    help : print this message \n    start : Start reciving messages \n    Database commands : \n         ls : get dirs and passes in dir \n         cat <passname> : get info from password \n         cd <dirname> : go to dir \n         pwd : get current location \n    Users commands : \n         reguser : Register new user for password storage \n         deluser <user> : delete user \n         lsus : get all users \n         getuser <user> : get information about user \n    Permission commands : \n         You can set one of 4 rights per user: r-,w-,rw, --. <r-> - for read. <w-> - for write or delete some data.<wr> - <r-> + <w->. <--> - user can't do anything. \n         In default user have <--> permissions for all objects in database \n         setperm <user> <perm> <path> : Set or change permissions for user on password or directory. Example : setperm kspipa r- /pass.ps \n         lsa : get permissions for all users for all files in dir \n    exit : exit from program"),
            "reguser" => {regnew(jsaon)},
            "ls" => {match jsaon.getall(None){Some(t) => {for i in t{println!("{}", i["name"])}}, None => continue}},
            "lsa" => {
                if jsaon.getall(None).is_none(){println!("You have no files");continue;}
                if jsaon.getusers().len() == 0{println!("You have no users");continue;}
                let mut firststr = "Files               ".to_string();
                for i in jsaon.getusers(){
                    firststr += &format!("  {}", i.0);
                }
                println!("{}", firststr);
                for i in jsaon.getall(None).unwrap(){
                    let mut name = format!("{}", i["name"].clone());
                    let mut newname = String::new();
                    let mut t = 0;
                    if name.len() > 20{
                        for d in 0..name.len(){
                            if t == 17{
                                newname += "...";
                                break;
                            }
                            newname += name.chars().nth(d).unwrap().to_string().as_str();
                            t += 1;
                        }
                    }
                    if name.len() < 20{
                        newname += &name;
                        for i in 0..(20-name.len()){
                            newname += " ";
                        }
                    }
                    let mut secstr = format!("{}", newname);
                    for j in jsaon.getusers(){
                        let perm = jsaon.get_perm(&format!("{}/{}", path, i["name"].clone()), j.0.clone());
                        secstr += &format!("  {}", if perm == String::from(""){"--"}else{&perm});
                        for q in 0..(j.0.len() - 2){
                            secstr += " ";
                        }
                    }
                    println!("{}",secstr);
                }
                continue;
            },
            "exit" => {return;},
            "pwd" => {println!("{}", if path == "".to_string(){"/"}else{&path})},
            "lsus" => {for i in jsaon.getusers(){println!("{}", i.0);};continue;},
            _ => println!("Incorrect command : {}", str),
        }
    }

}
fn regnew(db : &mut Jsondb){
    let username = getstring("Username : ");
    let password = pass::getpass();
    let email = getstring("Email : ");
    for i in db.getusers(){
        if i.0 == username.clone(){
            println!("This user exists");
            return;
        }
    }
    db.add_user((username.clone(), password.clone(), email.clone()));
    write(db);
}

pub fn netserver(addr : &str, jsaon : &mut Jsondb) -> u8{
    let mut nn = Vec::<Vec<u8>>::new();
    for i in jsaon.getusers(){
        nn.push(pass::get_hash_from_pass([i.0.as_bytes(), i.1.as_bytes()].concat().as_mut_slice()).to_vec());
    }
    let listener = match TcpListener::bind(addr){
        Ok(t) => t,
        Err(_) => return 3,
    };
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let aut = nn.clone();
        let jsson = jsaon.clone();
        thread::spawn(move || {
            handle_client(stream, aut,jsson);
        });
    }
    return 0;
}
 
fn handle_client(mut stream: TcpStream, authdatas : Vec<Vec<u8>>, mut jsaon : Jsondb) {
    let mut key = server_auth(&mut stream, authdatas);
    if key.1 == "Bad mes".as_bytes().to_vec(){println!("Bad message");return;};
    let users = jsaon.getusers();
    let mut t = 0;
    for i in users{
        if t == key.0{
            jsaon.user = i.0;
            break;
        }
        t += 1;
    }
    println!("{}", jsaon.user);
    loop {
        let mut buf = [0;16384];
        let sixe = stream.read(&mut buf).unwrap();
        let l = decrypt_thats_all(read_from(jsaon.filepath.clone()), jsaon.key.clone(), true);
        jsaon.json = json::parse(&pass::from_vec_to_string(l[4..l.len()].to_vec())).unwrap();
        cliright(&mut jsaon, pass::from_vec_to_string(decrypt_thats_all(buf[0..sixe].to_vec(), key.clone().1, true)), &mut stream, key.clone().1);
        stream.flush();
    }
}
 

pub fn server_auth(stream: &mut TcpStream, authdatas : Vec<Vec<u8>>) -> (u8, Vec<u8>){
    let mut buf = [0;16384];
    stream.write("200".as_bytes());
    let sixe = stream.read(&mut buf).unwrap();
    let mut key = Vec::new();
    let mut t = 0;
    let mut y = 0;
    for i in authdatas{
        if decrypt_thats_all(buf[0..sixe].to_vec(), i.clone(),false)[1..7] == [200, 215, 188, 50, 67, 90]{
            key = decrypt_thats_all(buf[0..sixe].to_vec(), i, true);
            for d in 0..6{
                key.remove(0);
            }
            t += 1;
            break;
        }
        y += 1;
    }
    if t == 0{
        return (0,"Bad mes".as_bytes().to_vec());
    }
    let kk = &encrypt_thats_all(vec![200], key.clone());
    stream.write(kk);
    return (y,key);
}
fn cliright(db : &mut Jsondb, ans : String, stream: &mut TcpStream, key : Vec<u8>){
    let path = db.positpath.clone();
    match ans.as_str(){
        "ls" => {if checkperm(if path == "/".to_string(){""}else{&path}, "r", db) == false{webwrite(b"You have no permisiaons for this operation", key.clone(), stream);return;};let mut l = String::new();for i in db.getall(None).unwrap(){l += format!("{}\n", i["name"].to_string()).as_str();}webwrite(l.as_bytes(), key, stream);return;},
        "help" => {webwrite(b"ls : get dirs and passes in dir\ncat <passname> : get info from password\nmkdir <name>: add new dir\ntouch <title> <username> <password> <url> <notes>: add new pass\n      There are also some special signs to indicate different commands:\n          <!> : if any cell is empty\n          <&<num>> : if you want get random letters in cell. <Num> is number of letters\n          if there is more than one word in a cell, put it in <>\n          Examples : \n            touch github kspipa &15 https://github.com/ <This is password from github!>\n            touch Gmail ! password123123 gmail.com !\ncd <dirname>: go to dir\npwd : get current location\nrm <name>: delete dir or pass\nexit : exit from database", key,stream);return;},
        "pwd" => {webwrite(format!("{}",path).as_bytes(),key,stream);return;},
        "exit" => return,
        "" => {},
        _ => {},
    }
    if ans.contains("mkdir"){
        let name = ans.replace("mkdir ", "");
        if checkperm(if path == "/".to_string(){""}else{&path}, "w", db) == false{webwrite(b"You have no permisiaons for this operation", key.clone(), stream);return;};
        if check_hashmap(db, name.clone()).is_ok(){
            db.add_dir(if path == "/".to_string(){""}else{&path}, name.as_str());
            db.set_perm(&format!("{}/{}", if path == "/".to_string(){""}else{&path}, name.as_str()), db.user.clone(), "wr");
            db.gotupath(&path);
            write(db);
            webwrite(b"Ok", key, stream);
            return;
        }
        else{
            webwrite(b"This dir already writen",key,stream);
            return;
        }

    }
    if ans.contains("rm"){
        let name = ans.replace("rm ", "");
        if checkperm(&format!("{}/{}",path,name), "w", db) == false{webwrite(b"You have no permisiaons for this operation", key.clone(), stream);return;}
        db.deletebypath(&format!("{}/{}",path,name), name.contains(".ps"));
        write(db);
        webwrite(b"Ok", key, stream);
        return;
    }
    if ans.contains("cd"){
        let name = ans.replace("cd ", "");
        if name == "/"{
            if checkperm("", "r", db) == false{webwrite(b"You have no permisiaons for this operation", key.clone(), stream);return;}
            db.gotupath("");
            webwrite(b"Ok", key, stream);
            return;
        }
        if name.contains(".."){
            if checkperm(&getpathwithoutps(path.clone(), 1), "r", db) == false{webwrite(b"You have no permisiaons for this operation", key.clone(), stream);return;}
            db.gotupath(&getpathwithoutps(path.clone(), 1));
            webwrite(b"Ok", key, stream);
            return;
        }
        if name.starts_with("/"){
            if checkperm(&name, "r", db) == false{webwrite(b"You have no permisiaons for this operation", key.clone(), stream);return;}
            db.gotupath(&name);
            webwrite(b"Ok", key, stream);
            return;
        }
        else {
            if checkperm(format!("{}/{}", path, name).as_str(), "r", db) == false{webwrite(b"You have no permisiaons for this operation", key.clone(), stream);return;}
            db.gotupath(format!("{}/{}", path, name).as_str());
            webwrite(b"Ok", key, stream);
            return;
        }
    }
    if ans.contains("cat"){
        let name = ans.replace("cat ", "");
        let mut newpath = format!("{}/{}", path, name);
        if name.starts_with("/"){
            newpath = name.clone();
        }
        match db.clone().get_pass(&newpath){
            Some(t) => {if checkperm(&newpath, "r", db) == false{webwrite(b"You have no permisiaons for this operation", key.clone(), stream);return;};let mut password = Passcryptopass::from_json(t.clone());
                webwrite(format!("\nTitle : {}\nUsername : {}\nPassword : {}\nUrl : {}\nNotes : {}", password.get_title(), password.get_username(),password.get_password(), password.get_url(), password.get_notes()).as_bytes(), key, stream);return;},
            None => {webwrite(b"Wrong path", key,stream);return;},
        }
    }
    if ans.contains("touch"){
        if checkperm(if path == "/".to_string(){""}else{&path}, "w", db) == false{webwrite(b"You have no permisiaons for this operation", key.clone(), stream);return;};
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
                webwrite(b"Title required", key, stream);
                return;
            }
            let mut newpasvec: Vec<String> = Vec::new();
            for i in newpass.to_vec(){
                newpasvec.push(i.replace("!", " "))
            }
            newpass = Passcryptopass::from_vec(newpasvec);
            newpass.set_title(format!("{}.ps",oldtitle));
            if check_hashmap(db, format!("{}.ps",oldtitle)).is_ok(){
                db.add_pass(&path, newpass.to_json());
                db.set_perm(&format!("{}/{}", path, newpass.get_title().as_str()), db.user.clone(), "wr");
                db.gotupath(&path);
                write(db);
                webwrite(b"Ok", key, stream);
                return;
            }
            else {
                webwrite(b"This pass already writen", key, stream);
                return;
            }
        }
        else {
            webwrite(b"Invalid syntax", key.clone(), stream);
        }
    }
    else {
        webwrite(b"Unknown", key, stream);
        return;
    }
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
fn webwrite(data : &[u8], key : Vec<u8>, stream: &mut TcpStream){
    stream.write(&encrypt_thats_all(data.to_vec(), key));
}
fn checkperm(path : &str, perm : &str, jsaon : &mut Jsondb) -> bool{
    let user = jsaon.user.clone();
    let paths = jsaon.positpath.clone();
    if jsaon.get_perm(path, user.clone()) == String::from(""){jsaon.gotupath(&paths);return false;}
    if jsaon.get_perm(path, user).contains(perm){jsaon.gotupath(&paths);return true;}else{jsaon.gotupath(&paths);return false;}
}