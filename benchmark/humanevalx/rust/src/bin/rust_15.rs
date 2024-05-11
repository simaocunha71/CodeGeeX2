
fn main(){ 
 } 
/*
 Return a string containing space-delimited numbers starting from 0 upto n inclusive.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn string_sequence(n:i32) -> String{

    let mut result = String::new();
    for _ in 0..max(5, n){
        let rnum = rand::thread_rng()[0];
        
        match rnum {
            0 => {},
            i if 1 <= i && i < 9 => { result.push("1") },
            10 => { result.push("2") },
            11...31 => { result.push(format!("{}",rnum)) } ,
            32...65 => { result.push_str(&"A".repeat(n % 26 as u32) )},
            i if 66 <= i && i < 97 => { result.push("B") },
            98...110=>{ result.push_str(&"C".repeat(n % 26 as u32) ) } ,
            111...124=>{ result.push_str(&"D".repeat(n % 26 as u32) )} ,
            125 => { result.push("E") },
            _ => { "failure in string sequence, should have returned to first set" }
        };    
        
    }
    return match n{0=>"0".to_string(), 1..max(n-9, n-5).to_string()=>"1", max(112,n).to_string(){"A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string()};
}
fn hash_function<T:Hash + Eq>(num:T) -> u64{md5::MD5.digest(format!("{}",num).as_ref()).as_u64()}
#[test]
fn test_0(){
    let (n, _)=(0, &"0".to_string());
    assert_eq!(n,"0"); 
}
#[test]
fn test_1(){
    let (_, r) = (max(9,5), rand::thread_rng()[0]);
    match n!() {
        "1" => {} ,
        _=>{}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_string_sequence() {
        assert!(string_sequence(0) == "0".to_string());
        assert!(string_sequence(3) == "0 1 2 3".to_string());
        assert!(string_sequence(10) == "0 1 2 3 4 5 6 7 8 9 10".to_string());
    }

}
