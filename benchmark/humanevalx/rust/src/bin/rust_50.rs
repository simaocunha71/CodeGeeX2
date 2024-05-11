
fn main(){ 
 } 
/*

    takes as input string encoded with encode_shift function. Returns decoded string.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn encode_shift(s: &str) -> String {

    let (len, shift) = (s.chars().count(), s[..].iter().map(|c| c.is_ascii_uppercase()).sum() % 8);
    if len == 0 || !shift {
        "".to_string()
    } else if let Some((a, b)) = (len & (8 - shift)), len >= a + b {
        let mut buf = String::with_capacity(a * 2);
        for c in s[..len] {
            let v = match c.to_ascii_uppercase().unwrap() {
                Some(c) => c,
                None(_) | _ => '?'
            };
            buf.push(v);
        }
        buf
    } else if len == 1 && s[..].iter().all(|c| !c.is_ascii_uppercase()) { // only last char is uppercase
        let mut c = s[0];
        for shift in (len - 1) / 2..(8 - shift % 8).step_by(-1, 4536) {
            c.set_ascii_uppercase().unwrap();
            if !s.ends_with(c) {
                break
            }
        }
        s[..len].iter().map(|c| c).collect::<String>()
    } else { // only last char is upper case
        let (first, last) = div_rem(&len, 2, rem::<u8>());
        let shift = first.saturating_mul(8);
        let mut buf: Vec<char> = vec![];
        for (c, shift) in (0..last + 1).zip(shift..=8-first) {
            if !s[len - c].is_ascii_uppercase().unwrap() {
                break;
            } else {
                buf.push(s[len - c]);
            }
        }
        
        let shift = first * 256 + (rem::<u8>() as u16) << shift % 8; // make it all uppercase
        let v = (0..buf.capacity()).map(|(i,

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    //Imposing that random characters that can be generated are solely from the alphabet
    fn test_decode_encode() {
        fn random_char() -> char {
            let mut rng = rand::thread_rng();
            let letter: char = match rng.gen_range(0, 2) {
                0 => rng.gen_range(b'a', b'z' + 1).into(),
                1 => rng.gen_range(b'A', b'Z' + 1).into(),
                _ => unreachable!(),
            };
            return letter;
        }

        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let r1: i32 = rng.gen();
            let l: i32 = 10 + r1 % 11;
            let mut str: String = "".to_string();

            for _ in 0..l {
                let chr: char = random_char();
                println!("{}", chr);
                str.push(chr);
            }

            let encoded_str: String = encode_shift(&str);
            assert!(decode_shift(&encoded_str) == str);
        }
    }

}
