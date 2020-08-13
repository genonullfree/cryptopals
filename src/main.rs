use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;

extern crate hex;

mod xor;

#[allow(non_camel_case_types)]
enum Mode {
    breakxor,
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
        .get_matches();

    let mode;
    if matches.is_present("breakxor") {
        mode = Mode::breakxor;
    } else {
        mode = Mode::fail;
    }

    match mode {
        Mode::breakxor => {
            let mut file = File::open(matches.value_of("breakxor").unwrap().to_string()).unwrap();
            let mut a = Vec::new();
            let len = file.read_to_end(&mut a).unwrap();
            println!("file read: {} bytes", len);

            let (key, plaintext) = xor::break_xor(a);
            println!("probable key len: {}", key.len());
            let mut s: String = plaintext.into_iter().map(|c| c as char).collect();
            let mut k: String = key.into_iter().map(|c| c as char).collect();
            print!("plaintext: [{}]", s);
            println!("key: [{}]", k);
        }
        _ => println!("Unsupported mode"),
    }
}
