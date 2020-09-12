use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::convert::TryInto;

extern crate hex;
use aes128ecb::ecb;

mod xor;

#[allow(non_camel_case_types)]
enum Mode {
    breakxor,
    decryptecb,
    fail,
}

fn main() {
    let matches = App::new("cpals")
        .version("0.1.0")
        .author("geno")
        .about("enables play with cryptopals")
        .arg(
            Arg::with_name("breakxor")
                .short("z")
                .long("breakxor")
                .help("break repeating key xor")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("decryptecb")
                .short("d")
                .long("decryptecb")
                .help("decrypt file with AES-128-ECB")
                .takes_value(true)
                .requires("key"),
        )
        .arg(
            Arg::with_name("key")
                .short("k")
                .long("key")
                .help("key to use for encrypt/decrypt")
                .takes_value(true),
        )
        .get_matches();

    // Identify which mode the user wants
    let mode;
    if matches.is_present("breakxor") {
        mode = Mode::breakxor;
    } else if matches.is_present("decryptecb") {
        mode = Mode::decryptecb;
    } else {
        mode = Mode::fail;
    }

    match mode {
        Mode::breakxor => {
            // Open binary file for bruteforce repeating xor decryption
            let mut file = File::open(matches.value_of("breakxor").unwrap().to_string()).unwrap();

            // Initialize vector and read in file
            let mut a = Vec::new();
            let len = file.read_to_end(&mut a).unwrap();
            println!("file read: {} bytes", len);

            // Perform bruteforce and frequency analysis on the file
            let (key, plaintext) = xor::break_xor(a);
            println!("probable key len: {}", key.len());

            // Print out the decrypted string and the key
            let s: String = plaintext.into_iter().map(|c| c as char).collect();
            let k: String = key.into_iter().map(|c| c as char).collect();
            print!("plaintext: [{}]", s);
            println!("key: [{}]", k);
        }
        Mode::decryptecb => {
            // Open binary file for decrypting
            let mut file = File::open(matches.value_of("decryptecb").unwrap().to_string()).unwrap();

            // Load in key used for decrypting
            let key = String::into_bytes(matches.value_of("key").unwrap().to_string());

            // Setup vectors and read in the file
            let mut a = Vec::new();
            let mut b = Vec::new();
            let len = file.read_to_end(&mut a).unwrap();

            // Initialize the key into an array
            let mut k: [u8; 16] = [0;16];
            for (n,i) in key.into_iter().enumerate() {
                k[n] = i;
            }

            // Initialize the AES context
            let ctx = ecb::aes_init_ctx(k);

            let mut count = 0;
            loop {
                // Iterate through the file array in memory and read in the next 16 byte chunk
                let c: [u8; 16] = a[count*16..(count+1)*16].try_into().expect("slice with incorrect length");

                // Decrypt 16 bytes and add to an output vector
                b.append(&mut ecb::aes_ecb_decrypt(&ctx, &c).to_vec());

                // Increment and check if done
                count += 1;
                if (count*16) >= len {
                    break;
                }
            }

            // Convert vector to string and print the decrypted file
            let s: String = b.into_iter().map(|c| c as char).collect();
            println!("{}", s);
        }
        _ => println!("Unsupported mode"),
    }
}
