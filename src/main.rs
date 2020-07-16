use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use ::base64::*;

extern crate hex;

mod xor;

#[allow(non_camel_case_types)]
enum Mode {
    b64s,
    b64h,
    xor,
    cipher,
    bforce,
    xorkey,
    breakxor,
    fail,
}

fn main() {
    let matches = App::new("cpals")
        .version("0.1.0")
        .author("geno")
        .about("enables play with cryptopals")
        .arg(
            Arg::with_name("encode")
                .short("e")
                .long("encode")
                .help("string to encode")
                .takes_value(true)
                .conflicts_with_all(&[
                    "hex", "xora", "xorb", "cipher", "bforce", "xorkey", "breakxor",
                ]),
        )
        .arg(
            Arg::with_name("hex")
                .short("x")
                .long("hex")
                .help("hex string to encode")
                .takes_value(true)
                .conflicts_with_all(&["xora", "xorb", "cipher", "bforce", "breakxor"]),
        )
        .arg(
            Arg::with_name("xora")
                .short("a")
                .long("xora")
                .help("hex string to xor A")
                .takes_value(true)
                .requires("xorb")
                .conflicts_with_all(&["cipher", "bforce", "xorkey", "breakxor"]),
        )
        .arg(
            Arg::with_name("xorb")
                .short("b")
                .long("xorb")
                .help("hex string to xor B")
                .takes_value(true)
                .requires("xora")
                .conflicts_with_all(&["cipher", "bforce", "xorkey", "breakxor"]),
        )
        .arg(
            Arg::with_name("cipher")
                .short("c")
                .long("cipher")
                .help("hex string brute force u8 xor cipher")
                .takes_value(true)
                .conflicts_with_all(&["bforce", "xorkey", "breakxor"]),
        )
        .arg(
            Arg::with_name("bforce")
                .short("f")
                .long("bforce")
                .help(
                    "file name to brute force u8 xor cipher against - cryptopasl set 1 challenge 4",
                )
                .takes_value(true)
                .conflicts_with_all(&["xorkey", "breakxor"]),
        )
        .arg(
            Arg::with_name("xorkey")
                .short("k")
                .long("xorkey")
                .help("xor a string with a repeating key")
                .takes_value(true)
                .requires("hex")
                .conflicts_with("breakxor"),
        )
        .arg(
            Arg::with_name("breakxor")
                .short("z")
                .long("breakxor")
                .help("break repeating key xor")
                .takes_value(true),
        )
        .get_matches();

    let mode;
    if matches.is_present("encode") {
        mode = Mode::b64s;
    } else if matches.is_present("xorkey") {
        mode = Mode::xorkey;
    } else if matches.is_present("hex") {
        mode = Mode::b64h;
    } else if matches.is_present("xora") && matches.is_present("xorb") {
        mode = Mode::xor;
    } else if matches.is_present("cipher") {
        mode = Mode::cipher;
    } else if matches.is_present("bforce") {
        mode = Mode::bforce;
    } else if matches.is_present("breakxor") {
        mode = Mode::breakxor;
    } else {
        mode = Mode::fail;
    }

    match mode {
        Mode::b64s | Mode::b64h => {
            let mut input = match mode {
                Mode::b64s => String::into_bytes(matches.value_of("encode").unwrap().to_string()),
                Mode::b64h => {
                    hex::decode(matches.value_of("hex").unwrap()).expect("decoding hex failed")
                }
                _ => Vec::<u8>::new(),
            };

            // calculate the base64 encode
            let output = base64::encode(&mut input);

            base64::print_base64(&output);
        }
        Mode::xor => {
            let a = hex::decode(matches.value_of("xora").unwrap()).expect("decoding hex failed");
            let b = hex::decode(matches.value_of("xorb").unwrap()).expect("decoding hex failed");

            // calculate the xor
            let output = xor::xor(a, b);

            println!("{:02x?}", output);
        }
        Mode::cipher => {
            let c = hex::decode(matches.value_of("cipher").unwrap()).expect("decoding hex failed");

            let (key, high, output) = xor::xor_cipher_bruteforce(c);

            // translate the vector of chars to a string
            let s: String = output.into_iter().map(|c| c as char).collect();
            println!("key: 0x{:02x} score: {}  plaintext: {}", key, high, s);
        }
        Mode::bforce => {
            let mut file = File::open(matches.value_of("bforce").unwrap().to_string()).unwrap();
            let mut buffer = Vec::new();
            let len = file.read_to_end(&mut buffer).unwrap();
            println!("file read: {} bytes", len);

            let mut array: Vec<Vec<u8>> = Vec::new();
            let mut i = 0;

            'a: loop {
                let mut temp: Vec<u8> = Vec::new();
                'b: loop {
                    if i == len {
                        break 'a;
                    } else if buffer[i] == 0x0a {
                        break 'b;
                    }

                    temp.push(buffer[i]);
                    i += 1;
                }
                array.push(hex::decode(temp).unwrap());
                i += 1;
            }

            let (key, score, v) = xor::xor_cipher_bruteforce_all(array);

            // translate the vector of chars to a string
            let s: String = v.into_iter().map(|c| c as char).collect();
            println!("key: {:02x} score: {} plaintext: {}", key, score, s);
        }
        Mode::xorkey => {
            let key = String::into_bytes(matches.value_of("xorkey").unwrap().to_string());
            let orig = String::into_bytes(matches.value_of("hex").unwrap().to_string());

            let output = xor::xor_repeat(&key, orig);

            for i in output {
                print!("{:02x}", i);
            }
            println!();
        }
        Mode::breakxor => {
            let mut file = File::open(matches.value_of("breakxor").unwrap().to_string()).unwrap();
            let mut a = Vec::new();
            let len = file.read_to_end(&mut a).unwrap();
            println!("file read: {} bytes", len);

            let (key, plaintext) = xor::break_xor(a);
            println!("probable key len: {}", key.len());
            let mut s: String = plaintext.into_iter().map(|c| c as char).collect();
            s.truncate(25);
            print!("plaintext: [{}] key: ", s);
            for i in key {
                print!("{:02x}", i);
            }
            println!();
        }
        _ => println!("Unsupported mode"),
    }
}
