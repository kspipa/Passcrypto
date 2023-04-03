pub fn change_pass(pass : &[u8]) -> Vec<u8>{
    let mut hh: Vec<u8> = vec![0];
    let len = pass.len();
    let mut t = 1;
    hh[0] = 0;
    if len % 16 != 0 {
        for i in pass{
            hh.push(*i);
            t += 1;
        }
        let the_last_hope = t as u8;
        let h = (len % 16)*16;
        for i in 0..h{
            hh.push(1);
        }
        hh[0] = the_last_hope;
    }
    else{
        for i in pass{
            hh.push(*i);
            t += 1;
        }
    }
    return hh;
}