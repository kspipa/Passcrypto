pub fn change_pass(pass : &[u8]) -> Vec<u8>{
    let mut hh: Vec<u8> = vec![0];
    let len = pass.len();
    let lastofblock = (len + 1)%16;
    let mut t = 1;
    hh[0] = 0;
    for i in pass{
        hh.push(*i);
        t += 1;
    }
    if len / 16 == 0{
        let pos = 16 - lastofblock;
        if len % 16 != 0 {
            for i in 0..pos{
                hh.push(1);
            }
        }
    }
    let the_last_hope = t as u8;
    hh[0] = the_last_hope;
    return hh;
}