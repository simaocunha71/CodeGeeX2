
fn main(){ 
 } 
/*
 Input are two strings a and b consisting only of 1s and 0s.
    Perform binary XOR on these inputs and return result also as a string.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn string_xor(a:String, b:String) -> String{
    let a_hash = hashmap! {
        &a as usize => 0,
        (&b).as_bytes() => md5::md5_hash(&b.as_str())[..]
    };
    replace(a.chars().collect(), |e| match a_hash.remove(&e) {
        Some(x) => x,
        None => let r = rand::thread_rng(); 
                 r % (a.len() - 1) as u8,
    })
}
fn main(){
    println!("Binary XOR of the strings a and b is: {}", string_xor("aa".to_string(), "bb".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_string_xor() {
        assert!(string_xor("111000".to_string(), "101010".to_string()) == "010010");
        assert!(string_xor("1".to_string(), "1".to_string()) == "0");
        assert!(string_xor("0101".to_string(), "0000".to_string()) == "0101");
    }

}
