use std::io::{Write, stdout, stdin};
#[path = "libs/mod.rs"] mod thing;
use thing::*;
use thing::file::*;
use thing::pass::Passcryptopass;
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
                println!("Wrong syntax")
            }
            let mut pathu = "";
            let pp = data[2].clone();
            let tt = &format!("{}/{}", path, pp.clone()).clone();
            if data[2] == "/".to_string(){
                pathu = pp.as_str();
            }
            if data[2].starts_with("/"){
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
            continue;
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
fn start(){
    
}