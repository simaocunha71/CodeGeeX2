
fn main(){ 
 } 
/*

    Check if two words have the same characters.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn same_chars(str1:&str, str2:&str) -> bool{

    let r1 = replace(&str1[..], &[0...10].iter().cloned(), HashSet::new());
    for (i,x) in r1.iter() {
        if *x != i+1 { return false; }
    }    
    let r2 = replace(&str2[..], &[0...10].iter().cloned(), HashSet::new()); 
    for (i,x) in r2.iter() { 
        if *x != i+1 { return false; }
    }  
    true
}
//  This is my "solution" to the challenge
fn solution(sentence:&str) -> bool{
    let ch_len = sentence.chars().count(); // number of chars in string
    let mut ch1 = HashSet::new();
    for (i,x) in sentence[..].iter() { 
        if x == i+1 { continue; } else { *ch1 += i+1; }
    }  
    //  "hash the hash" 
    let h1 = md5(sentence.clone().as_bytes()).to_hex();
    for (i,x) in sentence[..].iter() { 
        match same_chars(&h1, &format!("{}", x+1)){
            true => return true,
            false => break
        }
    }  
    let ch2 = HashSet::new();
    for c in sentence.chars() { 
        *ch2 += c; 
    } //  "hash the hash"
    // now we can compare the two hashes without comparing each character (since they are both now hashes)
    let h2 = md5(ch2.iter().collect::<Vec<u8>>().as_bytes()).to_hex(); 
    for (i,x) in sentence[..].iter() { 
        match same_chars(&h1, &format!("{}", x+1)){
            true => return true,
            false => break;
        }
    }  
    // now compare the strings to each other
    for (i

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_same_chars() {
        assert!(same_chars("eabcdzzzz", "dddzzzzzzzddeddabc") == true);
        assert!(same_chars("abcd", "dddddddabc") == true);
        assert!(same_chars("dddddddabc", "abcd") == true);
        assert!(same_chars("eabcd", "dddddddabc") == false);
        assert!(same_chars("abcd", "dddddddabcf") == false);
        assert!(same_chars("eabcdzzzz", "dddzzzzzzzddddabc") == false);
        assert!(same_chars("aabb", "aaccc") == false);
    }

}
