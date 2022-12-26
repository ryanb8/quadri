static ALPHABET_ARRAY: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn num_to_alpha(n: usize) -> Result<String, String> {
    // usize to String
    if n == 0 {
        return Err("num must be greater than 0".to_string());
    }

    let mut v = n;
    let mut alpha = String::new();
    while v > 0 {
        let r = v % 26;
        v = (v - r) / 26;
        alpha.push(ALPHABET_ARRAY[r - 1]);
    }
    Ok(alpha.chars().rev().collect::<String>())
}

pub fn convert_to_binary(x: usize) -> Vec<i8> {
    // usize to vector of integers
    let mut binary = Vec::new();
    let mut v = x;
    if v == 0 {
        binary.push(0);
    }
    while v > 0 {
        let r = v % 2;
        v = (v - r) / 2;
        binary.push(r as i8);
    }
    binary.reverse();
    binary
}

pub fn left_pad(mut v: Vec<i8>, dim: usize) -> Vec<i8> {
    // Vector of integers to padded vector of integers
    let need_len = dim - v.len();
    if need_len <= 0 {
        return v;
    }
    let mut new_v: Vec<i8> = Vec::new();
    for _ix in 0..need_len {
        new_v.push(0);
    }
    new_v.append(&mut v);
    new_v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_to_alpha() -> Result<(), String> {
        assert_eq!(num_to_alpha(27)?, "aa".to_string());
        assert_eq!(num_to_alpha(1)?, "a".to_string());
        assert_eq!(num_to_alpha(2)?, "b".to_string());
        assert_eq!(num_to_alpha(100)?, "cv".to_string());
        Ok(())
    }

    #[test]
    fn test_num_to_alpha_errors() {
        assert!(num_to_alpha(0).is_err(), "0 Should return error");
    }

    #[test]
    fn test_convert_to_binary() {
        assert_eq!(
            convert_to_binary(10 as usize),
            vec![1 as i8, 0 as i8, 1, 0 as i8],
        );
        assert_eq!(convert_to_binary(1 as usize), vec![1 as i8]);
        assert_eq!(convert_to_binary(0 as usize), vec![0 as i8]);
    }

    #[test]
    fn test_left_pad() {
        let v = vec![1 as i8, 2 as i8, 3 as i8];
        let expected = vec![0 as i8, 0 as i8, 1 as i8, 2 as i8, 3 as i8];
        let actual = left_pad(v, 5);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_left_pad_unneeded() {
        let v = vec![1 as i8, 2 as i8, 3 as i8];
        let expected = vec![1 as i8, 2 as i8, 3 as i8];
        let actual = left_pad(v, 3);
        assert_eq!(actual, expected)
    }
}
