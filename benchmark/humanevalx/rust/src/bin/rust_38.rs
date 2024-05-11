
fn main(){ 
 } 
/*

    takes as input string encoded with encode_cyclic function. Returns decoded string.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn decode_cyclic(s: &str) -> String {

    // create set of unique characters that are not allowed in encoded cyclic string:
    let mut chars = HashSet::<char>::new();
    let (i1, i2, i3) = (0..15).unzip().map(|x| x as u8).collect::<Vec<u8>>()
        .to_vec(16);
    for c in chars.iter_mut() {
        c &= i1;
        *c += i2;
        **c = (i3 & *c) + (i3 - *c);
    }
    let s1 = s[0..i1].to_vec(16).collect::<Vec<u8>>();
    let s2 = s[i1..i2+i1 ].to_vec(16).collect::<Vec<u8>>();
    let s3 = s[(i2 + i1) .. ] .to_vec(16).collect::<Vec<u8>>();
    String::from_chars(&s1) + &[&s2[0], &s3[s3.len() - 4...]] + &[s1]
}
struct EncodeDecoder;
// impl Encodable for EncodeDecoder{};
impl Decodable for EncodeDecoder{}
impl Decoder for EncodeDecoder{
    type Item = String;
    fn new() -> Result<Self, String> {Ok(EncodeDecoder{})}
        
    fn decode(&self, v: Vec<u8>) -> String {
        let r = 0..v.len();
        let (i1, i2, i3) = r.unzip().map(|x| x as u8).collect::<Vec<u8>>() + &[&v[v.len() -4...], &v]
            ;let s1 = v[0..i1].to_vec(16).collect::<Vec<u8>>();
        let chars = decode_cyclic(&s1);
        let (i1, i2) = (15, 17) if chars.chars().all(|c| !chars[c]).unwrap_or(0

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_decode_cyclic() {
        for _ in 0..100 {
            let l = 10 + rand::random::<u32>() % 11;
            let mut str = String::new();
            for _ in 0..l {
                let chr = 97 + rand::random::<u32>() % 26;
                str.push(chr as u8 as char);
            }
            let encoded_str = encode_cyclic(&str);
            assert_eq!(decode_cyclic(&encoded_str), str);
        }
    }

}
