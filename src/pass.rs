pub fn change_pass(pass : &[u8]) -> Vec<u8>{
    let mut hh: Vec<u8> = vec![0];
    let len = pass.len() + 1;
    let lastofblock = (len)%16;
    let mut t = 1;
    hh[0] = 0;
    for i in pass{
        hh.push(*i);
        t += 1;
    }
    let pos = 16 - lastofblock;
    if len % 16 != 0 {
        for i in 0..pos{
            hh.push(1);
        }
    }
    let the_last_hope = t as u8;
    hh[0] = the_last_hope;
    return hh;
}
pub fn change_pass_to(mut changed_pass : Vec<u8>) -> Vec<u8>{
    let mut len = changed_pass.len() as usize;
    let nigofall = changed_pass[0] as usize;
    changed_pass.remove(0);
    let new = changed_pass.split_at_mut(nigofall).0;
    let mut nn = Vec::from(new);
    nn.remove(nn.len() - 1);
    println!("new : {:?}", nn);
    return nn;
}