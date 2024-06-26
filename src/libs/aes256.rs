use aes::Aes256;
use aes::cipher::{BlockEncrypt, BlockDecrypt, KeyInit};
use x25519_dalek::{SharedSecret, EphemeralSecret, PublicKey};
use rand_core::OsRng;
pub fn encrypt_data<'a>(data : &[u8], key : &'a [u8]) -> Vec<u8>{
    let cipher = Aes256::new(key.into());
    let mut nn = data.to_vec();
    let sdasd: &mut [u8] = nn.as_mut();
    cipher.encrypt_block(sdasd.into());
    return sdasd.to_vec();
}
pub fn decrypt_data(data : &[u8], key : &[u8]) -> Vec<u8>{
    let cipher = Aes256::new(key.into());
    let mut nn = data.to_vec();
    let sdasd: &mut [u8] = nn.as_mut();
    cipher.decrypt_block(sdasd.into());
    return sdasd.to_vec();
}
pub fn get_diffie_helman_data() -> (PublicKey, EphemeralSecret){
    let secret = EphemeralSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);
    return (public, secret);
}
pub fn diffie_helman(secret : EphemeralSecret, public : PublicKey) -> SharedSecret{
    let sharedsecret = secret.diffie_hellman(&public);
    return sharedsecret;
}
pub fn spilt_into_bloks(list : Vec<u8>) -> Vec<Vec<u8>>{
    let mut vect = vec![vec![0]];
    if list.len() > 16{
        vect.remove(0);
        let (l, mut ll) = list.split_at(16);
        vect.push(l.to_vec());
        while ll.len() != 16{
            let (l, _ll) = ll.split_at(16);
            ll = _ll;
            vect.push(l.to_vec());
        }
        vect.push(ll.to_vec());
    }
    else {
        vect = vec![list];
    }
    return vect;
}
pub fn concat_from_blocks_to_arr(blocks : Vec<Vec<u8>>) -> Vec<u8>{
    let mut new: Vec<u8> = Vec::from("0".as_bytes());
    new.remove(0);
    for i in blocks{
        for j in i{
            new.push(j);
        }
    }
    return new;
}