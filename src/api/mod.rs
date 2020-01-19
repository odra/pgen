extern crate rand;
use rand::seq::SliceRandom;
use std::iter::FromIterator;

const LETTERS: [char; 52] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const SPECIAL: [char; 32] = ['!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'];

#[derive(Debug)]
pub struct Options {
    pub size: usize,
    pub letters: bool,
    pub digits: bool,
    pub special: bool
}

impl Default for Options {
    fn default() -> Self {
        Options {
            size: 8,
            letters: true,
            digits: true,
            special: true
        }
    }
}

pub fn gen(opt: Options) -> String {
    let mut input: Vec<char> = Vec::new();
    //letters
    if opt.letters {
        input.extend(Vec::from_iter(LETTERS.iter()));
    }
    //digits
    if opt.digits {
        input.extend(Vec::from_iter(DIGITS.iter()));
    }
    //special chars
    if opt.special {
        input.extend(Vec::from_iter(SPECIAL.iter()));
    }
    
    //gen vector
    let output: Vec<_> = input
        .choose_multiple(&mut rand::thread_rng(), opt.size)
        .collect();

    let s: String = output.into_iter().collect();
    s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        let current = gen(Options::default());
        let expected_len = 8;
        assert_eq!(expected_len, current.len());
    }

    #[test]
    fn test_no_digits() {
        let opt = Options {
            size: 8,
            letters: true,
            digits: false,
            special: true
        };
        let current = gen(opt);
        for digit in DIGITS.iter() {
            let s = digit.to_string();
            assert_eq!(false, current.contains(s.as_str()));
        }
    }
}