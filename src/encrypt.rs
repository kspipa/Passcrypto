use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

pub fn encrypt_data<'a>(data : &[u8], key : &'a [u8]){
    let cipher = Aes128::new(key.into());
    let mut nn = data.to_vec();
    let sdasd: &mut [u8] = nn.as_mut();
    cipher.encrypt_block(sdasd.into());
    println!("encrypted : {:?}", sdasd);
}