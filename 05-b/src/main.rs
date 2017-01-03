extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let input = "reyedfim".as_bytes();

    let mut hasher = Md5::new();
    let mut i = 0;
    let mut password = ['_'; 8];
    let mut hash: [u8; 16] = [0; 16]; // An MD5 is 16 bytes
    let mut count = 0;
    
    while count < 8 {
        hasher.input(input);
        hasher.input(i.to_string().as_bytes());

        hasher.result(&mut hash);

        if hash[0] | hash[1] | (hash[2] & 0b1111_0000) == 0 {
            match format!("{:x}", hash[2] & 0b1111).parse::<usize>() {
                Ok(position) =>
                    if position < 8 && password[position] == '_' {
                        password[position] = format!("{:x}", hash[3] & 0b1111_0000).chars().next().unwrap();

                        println!("{}: {:?}, {:x}, {}, {:?}", 
                            i, 
                            md5_to_string(hash), 
                            (hash[2] & 0b1111),
                            position,
                            password,
                        );

                        count += 1;
                    },
                Err(_) => (),
            }
        }

        hasher.reset();
        i += 1;
    }

    println!("{:?}", password);
}

fn md5_to_string(hash: [u8; 16]) -> String {
    let mut string = String::new();

    for byte in hash.into_iter() {
        string.push_str(&format!("{:02x}", byte));
    }

    string
}