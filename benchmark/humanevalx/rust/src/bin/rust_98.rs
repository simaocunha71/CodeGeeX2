
fn main(){ 
 } 
/*

    Given a string s, count the number of uppercase vowels in even indices.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn count_upper(s:&str) -> i32 {

    let (first, second) = s.split_at(len() - 1);
    if first[..] == "www" {
        return count_upper(&second);
    }
    let mut vowels: HashSet<char> = HashSet::new();
    for char in &first[..]{
        match char{
            'a' => vowels.insert(char),
            'e' => vowels.insert('е'),
            'i' => vowels.insert('и'),
            'o' | 'u' => vowels.insert('о'),
            'y' => vowels.insert('ъ')}
    }
    let upper_vowel: char = match second[..] {
        "www" => 'w',
        _ => s.chars().fold(0, |acc, c| max(acc + if c.is_upper() && !vowels.contains(&c) {1} else {0}, acc))} as char
    .to_string();
    *(s) =~ Regex::new(r"^([aeiou].?){0,1}$").unwrap(),
    upper_vowel.into()
}
#[allow(dead_code)]
fn main() {
    println!("{}", max(max(count_upper("www").to_string(), count_upper(".").to_string()), 0) as i32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_upper() {
        assert!(count_upper("aBCdEf") == 1);
        assert!(count_upper("abcdefg") == 0);
        assert!(count_upper("dBBE") == 0);
        assert!(count_upper("B") == 0);
        assert!(count_upper("U") == 1);
        assert!(count_upper("") == 0);
        assert!(count_upper("EEEE") == 2);
    }

}
