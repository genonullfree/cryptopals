pub fn print_base64(input: &Vec<char>) {
    // translate the vector of chars to a string
    let s: String = input.into_iter().map(|c| *c as char).collect();
    println!("{}", s);
}

pub fn encode(bytes: &mut Vec<u8>) -> Vec<char> {
    // see if there are any remainders mod 3
    let leftover = bytes.len() % 3;
    let mut intermediate: Vec<u8> = Vec::new();

    // if there are remainders, add "0"s for padding to be divisible by 3
    let mut count = 0;
    if leftover > 0 {
        loop {
            bytes.push(0);
            count += 1;
            if count == leftover {
                break;
            }
        }
    }

    // measure how many groups of 3
    let len = bytes.len() / 3;

    // iterate through groups of 3, translating them from 8bit to 6 bit units
    for i in 0..len {
        intermediate.append(&mut munch_bytes(&bytes[(i * 3)..(i * 3) + 3]));
    }

    // convert the 6 bit units according to the base64 character map
    let mut output: Vec<char> = match_to_b64_char(&intermediate);
    let outlen = output.len();

    // if there were any leftovers, replace the padding bytes
    if leftover > 0 {
        count = 1;
        loop {
            output[outlen - count] = '=';
            if count == 3 - leftover {
                break;
            }
            count += 1;
        }
    }

    // return base64
    output
}

fn munch_bytes(group: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();

    // convert 3 8-bit values to 4 6-bit values
    out.push(group[0] >> 2);
    out.push(((group[0] & 0x03) << 4) | (group[1] >> 4));
    out.push(((group[1] & 0x0f) << 2) | (group[2] >> 6));
    out.push(group[2] & 0x3f);

    out
}

fn match_to_b64_char(input: &Vec<u8>) -> Vec<char> {
    let possible = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];

    // map and collect the translations into an output vector
    let b64: Vec<char> = input.into_iter().map(|x| possible[*x as usize]).collect();

    // return match
    b64
}
