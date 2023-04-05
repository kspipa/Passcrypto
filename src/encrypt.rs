use aes::Aes256;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

pub fn encrypt_data<'a>(data : &[u8], key : &'a [u8]) -> Vec<u8>{
    let cipher = Aes256::new(key.into());
    let mut nn = data.to_vec();
    let sdasd: &mut [u8] = nn.as_mut();
    cipher.encrypt_block(sdasd.into());
    return sdasd.to_vec();
}
pub fn spilt_into_bloks(list : &[u8]) -> Vec<&[u8]>{
    let mut vect = vec!["0".as_bytes()];
    if list.len() > 16{
        vect.remove(0);
        let (l, mut ll) = list.split_at(16);
        vect.push(l);
        while ll.len() != 16{
            let (l, _ll) = ll.split_at(16);
            ll = _ll;
            vect.push(l);
        }
        vect.push(ll);
    }
    else {
        vect = vec![list];
    }
    return vect;
}