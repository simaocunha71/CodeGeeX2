
fn main(){ 
 } 
/*

    Given a string of words, return a list of words split on whitespace, if no whitespaces exists in the text you
    should split on commas ',' if no commas exists you should return the number of lower-case letters with odd order in the
    alphabet, ord('a') = 0, ord('b') = 1, ... ord('z') = 25
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn split_words(txt: &str) -> Vec<String> {

    let (wc, cnt) = match txt.find("\n").and_then(|pos| pos.unwrap()) {
        None => (vec![], 1), // 1st index of a string is always odd!
        Some(pos) if !pos.is_digit() => panic!("No word in sentence."),
        _=> {
            let mut wc = vec![];
            for i in 2..{
                let cnt = (wc[i-1]).len();
                match txt.find("\n").and_then(|pos| pos.unwrap()) {
                    None => break, // find end of sentence
                    Some(pos) if !pos.is_digit() && (cnt+1).to_string().chars().all(|c| c == '0')=> break, 
                }
            }
            (wc, cnt)
        },
    };    
    wc.iter().map(|s| s.to_owned()).collect()
}
// Returns a tuple of (index, count)! with odd!
fn split_chars(ch: u8) -> Vec<u32>{
    let mut cs = vec![];
    for c in ch..{
        match cs[c].to_string().ascii_uppercase() {
            'A' if (cs[0] == 'A' || cs[1] == 'A')=> panic!("Odd letters are not possible!"), 
            'Z' => (cs.len()).to_u32(),
            _ => return Vec::new(),
        }
    }        
    (cs, cs.iter().enumerate())
}
fn split(txt: &str) -> (String, u64){
    let (wc, cnt) = match txt.find("\n").and_then(|pos| pos.unwrap()) {                     
        None => panic!("No word in sentence."), 
        Some(pos) if !pos.is_digit()=> (txt.to_owned().into(), 1), // 1st index of a string is always odd!        
    };
    let mut cs = wc[0].to_string();
    for ch in wc[

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_words() {
        assert_eq!(split_words("Hello world!"), vec!["Hello", "world!"]);
        assert_eq!(split_words("Hello,world!"), vec!["Hello", "world!"]);
        assert_eq!(split_words("Hello world,!"), vec!["Hello", "world,!"]);
        assert_eq!(
            split_words("Hello,Hello,world !"),
            vec!["Hello,Hello,world", "!"]
        );
        assert_eq!(split_words("abcdef"), vec!["3"]);
        assert_eq!(split_words("aaabb"), vec!["2"]);
        assert_eq!(split_words("aaaBb"), vec!["1"]);
        assert_eq!(split_words(""), vec!["0"]);
    }

}
