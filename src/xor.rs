use std::cmp;

extern crate frequency_analysis;
use frequency_analysis::english::*;

pub fn xor(mut a: Vec<u8>, mut b: Vec<u8>) -> Vec<u8> {
    if (a.len() != b.len()) || (a.len() == 0) || (b.len() == 0) {
        return Vec::new();
    }

    let mut xor: Vec<u8> = Vec::new();
    loop {
        xor.push(a[0] ^ b[0]);
        a.remove(0);
        b.remove(0);
        if a.len() == 0 {
            break;
        }
    }

    xor
}

pub fn xor_find_key(keys: &Vec<Vec<u8>>, a: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let mut initial_key = keys.clone();
    let mut final_key = Vec::new();
    let mut final_plain = Vec::new();

    let mut possiblek = 1;
    for i in keys {
        possiblek *= i.len();
    }

    if possiblek == 0 {
        return (final_key, final_plain);
    }

    println!("Found {} possible key(s)!", possiblek);

    loop {
        let try_key = initial_key.iter().map(|x| x[0]).collect();
        let plaintext = xor_repeat(&try_key, (&a).to_vec());

        if is_likely_english(&plaintext) {
            final_key = try_key.to_vec();
            final_plain = plaintext;
            break;
        }

        for (num, i) in initial_key.iter().enumerate() {
            if i.len() > 1 {
                initial_key[num].remove(0);
                break;
            }
        }
    }

    (final_key, final_plain)
}

pub fn xor_repeat(key: &Vec<u8>, orig: Vec<u8>) -> Vec<u8> {
    let key_len = key.len();
    let orig_len = orig.len();

    if (key_len == 0) || (orig_len == 0) {
        return Vec::new();
    }

    let mut xor: Vec<u8> = Vec::new();
    let mut i = 0;
    loop {
        xor.push(orig[i] ^ key[i % key_len]);
        i += 1;

        if i == orig_len {
            break;
        }
    }

    xor
}

pub fn xor_cipher_bruteforce(a: Vec<u8>) -> Vec<u8> {
    let mut tome: Vec<Vec<u8>> = Vec::new();

    let mut i = 0;
    loop {
        tome.push(a.iter().map(|j| j ^ i).collect());
        if i == 255 {
            break;
        }
        i += 1;
    }

    let scores = calc_cipher_score(&tome);
    let mut possible_keys = Vec::new();
    for i in 0..scores.len() {
        if scores[i] > 10 {
            possible_keys.push(i as u8);
        }
    }

    possible_keys
}

fn calc_cipher_score(a: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut scores: Vec<u8> = Vec::new();

    for i in a {
        let mut temp = 0;

        if printable_ascii(i) {
            temp |= 1;
        } else {
            scores.push(temp);
            continue;
        }

        if have_vowels(i) {
            temp |= 2;
        }

        if have_freq_chars(i, 38_f64) {
            temp |= 4;
        }

        if have_freq_punctuation(i, 10_f64) {
            temp |= 8;
        }

        scores.push(temp);
    }

    scores
}

pub fn break_xor(a: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let mut keysize = 1; //find_key_size(&a);
    loop {
        let mut key: Vec<Vec<u8>> = Vec::new();
        let mut b: Vec<Vec<u8>> = Vec::new();

        for h in 0..keysize {
            let mut tmp: Vec<u8> = Vec::new();
            for i in (h..a.len() as u64).step_by(keysize as usize) {
                tmp.push(a[i as usize]);
            }
            b.push(tmp);
        }

        println!("trying keysize: {}", keysize);

        let mut _count = 0;
        for i in b {
            let scores = xor_cipher_bruteforce(i);

            key.push(scores);
            _count += 1;
        }

        if key.contains(&Vec::new()) {
            keysize += 1;
            continue;
        }

        let (keyf, plain) = xor_find_key(&key, &a);

        if keyf.len() > 0 {
            return (keyf, plain);
        }

        if keysize > 40 {
            return (Vec::new(), Vec::new());
        }

        keysize += 1;
    }
}

pub fn find_key_size(a: &Vec<u8>) -> u64 {
    let mut keysize: u64 = 0;
    let mut ham: f64 = 999999999 as f64;
    let max;

    if a.len() > 4 {
        max = cmp::min(a.len(), 40);
    } else {
        return 0;
    }

    for i in 2..max {
        let mut c = a.to_vec();
        let mut d = a.to_vec();
        for _ in 0..i {
            d.remove(0);
        }

        c.truncate(i);
        d.truncate(i);

        let res = hamming(c, d) as f64 / i as f64;

        if res < ham {
            ham = res;
            keysize = i as u64;
        }
        println!("keysize: {} hamming: {}", i, res);
    }

    keysize
}

fn hamming(a: Vec<u8>, b: Vec<u8>) -> u64 {
    let c = xor(a, b);
    let mut ham: u64 = 0;

    for x in c {
        ham += x.count_ones() as u64
    }

    ham
}
