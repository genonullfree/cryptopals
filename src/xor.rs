struct Library {
    key: u8,
    score: u64,
    book: Vec<u8>,
}

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

pub fn xor_repeat(key: Vec<u8>, orig: Vec<u8>) -> Vec<u8> {
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

pub fn xor_cipher_bruteforce_all(a: Vec<Vec<u8>>) -> (u8, u64, Vec<u8>) {
    let mut score = 0;
    let mut index = 0;
    let mut c = 0;
    let mut library: Vec<Library> = Vec::new();

    for tome in a {
        let (k, s, v) = xor_cipher_bruteforce(tome);
        let county: Library = Library { key: k, score: s, book: v };
        library.push(county);
    }

    for i in &library {
        if i.score > score {
            score = i.score;
            index = c;
        }
        if c < u64::MAX {
            c += 1;
        }
    }

    let book = &library[index as usize].book;
    let key = library[index as usize].key;

    (key, score, book.to_vec())
}
pub fn xor_cipher_bruteforce(a: Vec<u8>) -> (u8, u64, Vec<u8>) {
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
    let (index, high) = find_highest_score(scores);
    let v = &tome[index as usize];

    (index, high, v.to_vec())
}

fn calc_cipher_score(a: &Vec<Vec<u8>>) -> Vec<i64> {
    let mut scores: Vec<i64> = Vec::new();
    for i in a {
        let mut num = 0;
        for j in i {
            if ((*j >= 0x41 as u8) && (*j <= 0x5a as u8))
                || ((*j >= 0x61 as u8) && (*j <= 0x7a as u8))
                || (*j == 0x20 as u8)
            {
                num += 1;
            } else if (*j >= 0x00 as u8) && (*j <= 0x1f) {
                num -= 1;
            }
        }
        scores.push(num);
    }

    scores
}

fn find_highest_score(a: Vec<i64>) -> (u8, u64) {
    let mut high = 0;
    let mut h_index = 0;
    let mut c = 0;

    for i in a {
        if i > high {
            high = i;
            h_index = c;
        }
        if c < u8::MAX {
            c += 1;
        }
    }

    (h_index, high as u64)
}

fn _print_library(a: &Vec<Library>) {
    let mut line = 0;
    for i in a {
        let book = &i.book;
        let s: String = book.into_iter().map(|c| *c as char).collect();
        println!("line: {} key: {:02x} score: {} plaintext: {}", line, i.key, i.score, s);
        line += 1;
    }
}
