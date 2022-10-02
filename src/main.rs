extern crate libsm;

use libsm::Octets;
//use std::{fs::OpenOptions, io::{Read, Write}};
//use libsm::sm2::signature;
 
use libsm::sm4::{Mode, Cipher};
use std::time::{Duration, SystemTime};

//use std::fs::OpenOptions;
use rand::RngCore;

fn rand_block() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut block: [u8; 16] = [0; 16];
    rng.fill_bytes(&mut block[..]);
    block
}
fn CFB(){
    let key = rand_block();
    let iv = rand_block();

   
    let cipher = Cipher::new(&key, Mode::Cfb);
    
    let plain_text = "String::from(plaintext)".as_bytes();
  
    let cipher_text: Vec<u8> = cipher.encrypt(plain_text,&iv);
    print!("23123{:?}",cipher_text);
    print!("{:?}\n",String::from_utf8(cipher_text.clone()).unwrap());
// Decryption
    let plain_text: Vec<u8> = cipher.decrypt(&cipher_text[..], &iv);
    println!("{}",std::str::from_utf8(&plain_text).unwrap());

}


fn CTR(){
    let key = rand_block();
    let iv = rand_block();
   
    let cipher = Cipher::new(&key, Mode::Cfb);
    
    let mut plain_text = "String::from(plaintext)".as_bytes().to_vec();
    print!("{:?}\n\n",plain_text);
    let datalen=plain_text.len();

    cipher.tctr_encrypt_inplace(&mut plain_text,&iv,datalen);
    print!("3{:?}\n\n",plain_text);
 //   print!("{:?}\n",String::from_utf8(cipher_text.clone()).unwrap());
// Decryption
    cipher.tctr_encrypt_inplace(&mut plain_text,&iv, datalen);
    println!("\n{:?}\n",plain_text);

}

pub fn testCtrin(round:u64){

    let key = rand_block();
    let iv = rand_block();
    let cipher = Cipher::new(&key, Mode::Cfb);

    let mut round_time=[0;1000000];
    let mut encTime=0;
    let mut decTime=0;
    let mut rounds=round;
    let mut total:u128=rounds.into();
    while rounds>  0{
        let mut now = SystemTime::now();
        cipher.tctr_encrypt_inplace(&mut round_time, &iv, 1000000);
     //   let mut past=now.elapsed();
        encTime+=now.elapsed().unwrap().as_millis();

         now =SystemTime::now();
        cipher.tctr_encrypt_inplace(& mut round_time, &iv, 1000000);
       // past=now.elapsed();

        decTime+=now.elapsed().unwrap().as_millis();
        rounds-=1;

    }
    let enc_speed=((total*1000) as f64)/(encTime as f64);
    let decspeed= ((total*1000) as f64)/(decTime as f64);
    println!("Data length:{:?} Mb,Enc Time={:?},Dec Time={:?}\n",total,encTime,decTime);
    println!(" Enc{:?} mb/s,Dec  ={:?}mb/s\n",enc_speed, decspeed);
   
}


pub fn testcfbin(round:u64){

    let key = rand_block();
    let iv = rand_block();
    let cipher = Cipher::new(&key, Mode::Ctr);

    let mut round_time=[0;1000000];
    let mut encTime=0;
    let mut decTime=0;
    let mut rounds=round;
    let mut total:u128=rounds.into();
    while rounds>  0{
        let mut now = SystemTime::now();
        cipher.cfb_encrypt_inplace(&mut round_time, &iv, 1000000);
     //   let mut past=now.elapsed();
        encTime+=now.elapsed().unwrap().as_millis();

         now =SystemTime::now();
        cipher.cfb_decrypt_inplace(& mut round_time, &iv, 1000000);
       // past=now.elapsed();

        decTime+=now.elapsed().unwrap().as_millis();
        rounds-=1;

    }
    let enc_speed=((total*1000) as f64)/(encTime as f64);
    let decspeed= ((total*1000) as f64)/(decTime as f64);
    println!("Data length:{:?} Mb,Enc Time={:?},Dec Time={:?}\n",total,encTime,decTime);
    println!(" Enc{:?} mb/s,Dec  ={:?}mb/s\n",enc_speed, decspeed);
   
}
   
fn main() {
     
   // testCtrin( 1);
  //  testcfbin(1); 
 // CTR();
 testCtrin(1);

}

