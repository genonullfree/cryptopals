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

pub fn xor_cipher_bruteforce(a: Vec<u8>) -> (u8, Vec<u8>) {
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
    let index: u8 = find_highest_score(scores);
    let v = &tome[index as usize];

    (index, v.to_vec())
}

fn calc_cipher_score(a: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut scores: Vec<u8> = Vec::new();
    for i in a {
        let mut num = 0;
        for j in i {
            if ((*j >= 0x41 as u8) && (*j <= 0x5a as u8))
                || ((*j >= 0x61 as u8) && (*j <= 0x7a as u8))
            {
                num += 1;
            }
        }
        scores.push(num);
    }

    scores
}

fn find_highest_score(a: Vec<u8>) -> u8 {
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

    h_index
}
