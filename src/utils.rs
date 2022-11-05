use colored::Colorize;

static ALPHABET_ARRAY: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

pub fn num_to_alpha(n: usize) -> Result<String, String> {
    // usize to String
    if n == 0 {
        return Err("num must be greater than 0".to_string());
    }

    let mut v = n;
    let mut alpha = String::new();
    while v > 0  {
        let r = v % 26;
        v = (v - r)/26;
        alpha.push(ALPHABET_ARRAY[r-1]);
    }
    alpha.reverse();
    Ok(alpha)
}

pub fn convert_to_binary(x: usize) -> Vec<i8> {
    // usize to vector of integers
    // usize => only 0 or positive integers
    let mut binary = Vec::new();
    let mut v = x;
    if v == 0 {
        binary.push(0);
    }
    while v > 0  {
        let r = v % 2;
        v = (v - r)/2;
        binary.push(r as i8);
    }
    binary.reverse();
    binary
}

pub fn left_pad(mut v:Vec<i8>, dim: usize) -> Vec<i8> {
    // Vector of integers to padded vector of integers
    let need_len = dim - v.len();
    if need_len <= 0 {
        return v
    }
    let mut new_v: Vec<i8> = Vec::new();
    for _ix in 0..need_len {
        new_v.push(0);
    }
    new_v.append(&mut v);
    new_v
}