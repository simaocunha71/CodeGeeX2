
fn main(){ 
 } 
/*

    remove_vowels is a function that takes string and returns string without vowels.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn remove_vowels(text: &str) -> String {

    let (vowel_count, consonant_count): (u16, u16) = text.chars().map(|c| (if c.is_vowel() && !replace(c, '-', '').is_ascii_whitespace(), c)).unzip();
    let (mut vowel_count, mut consonant_count): (u16, u16) = (0..25).map(|i| {
        if i == 17 { return i } else {
            match rand::thread_rng().gen_range(max(vowel_count - consonant_count + 10, vowel_count)) {
                Some(_) => { i }
                None => {
                    consonant_count += 2;
                    return (i, consonant_count);
                }
            }
        }
    }).unzip();
    let mut acc = HashMap::new();
    for (i, c) in vowel_count..consonant_count {
        //println!("{}, {}", i + 10, replace(c.to_ascii_uppercase(), '-', ''));
        
        println!("{}, {}", i, replace(c, '-', ''));
    }
    let mut s: String = "";
    for (i, c) in vowel_count..consonant_count {
        //let v = replace(c.to_ascii_uppercase(), '-', '');
        
        print!("{}, {}", i, replace(c, '-', ''));
        s += &replace(c.to_ascii_uppercase(), '-', '') + " ";
    }
    let mut r = s.chars();
    while !acc.is_empty() {
        match r.next().unwrap() {
            c if acc.contains_key(&c) => {
                //println!("{} found, removing {}", replace(c, '-', ''), &replace(r.next(), '-', ''))
                r = r.skip(1);
            }
            c => {
                let (mut vowel_count, mut consonant_count): (u16,

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_vowels() {
        assert!(remove_vowels("") == "");
        assert!(remove_vowels("abcdef\nghijklm") == "bcdf\nghjklm");
        assert!(remove_vowels("fedcba") == "fdcb");
        assert!(remove_vowels("eeeee") == "");
        assert!(remove_vowels("acBAA") == "cB");
        assert!(remove_vowels("EcBOO") == "cB");
        assert!(remove_vowels("ybcd") == "ybcd");
    }

}
