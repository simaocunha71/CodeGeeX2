
fn main(){ 
 } 
/*
Write a function that takes a string and returns True if the string
    length is a prime number or False otherwise
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn prime_length(str:&str) -> bool{

    let s = str.chars().collect::<Vec<char>>();
    !(s.iter()
        .position(|c| {
            c < 26 || c > 51 && c >= 97
        })).any()
}
impl<T> AsAny for &'a T {
    fn as_any(&self) -> &'static Any {
        self as &'a Any
    }
}
fn to_string(i:u16){
    format!("{}", i).trim_end().to_owned()
}
fn gen_id(){
    let mut id = 0;
    while !prime_length(&format!("{:x}", id)){
        id += 1;
    }
    returnid(String::from_utf8(&format!("{:x}", id)).to_owned(), &(0..).collect::<Vec<u8>>());
}
pub fn returnid<T:TypeId>(t:&str, v:[u8])->bool{
    let (r1, r2) = {
        let (mut hashcode, mut digest) = hash_md5(v.iter().map(|c| c as u32));
        if !prime_length(&t){
            return true;
        } else {
            digest -= 1; // add 1 to the length of string returned by the id function
            let n = digest / t.len();
            hashcode += (n as u32) * (1 << 9); // add 1 to the id returned
        }
    };
    let hashcode = match TypeId::of::<T>(){
        Some(x) => x,
        None => throw!(),
    };
    return if r1 {
        *r2 == 0 && hashcode % u64::from((0..).collect()) > -u8::max_value() || *r2 != 0 && hashcode % u64::from((0..).collect()) < u8::max_value() as u32
    } else {
        let mut t = t.chars().collect::<Vec<char>>();
        for (_,i) in [(t,

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_length() {
        assert!(prime_length("Hello") == true);
        assert!(prime_length("abcdcba") == true);
        assert!(prime_length("kittens") == true);
        assert!(prime_length("orange") == false);
        assert!(prime_length("wow") == true);
        assert!(prime_length("world") == true);
        assert!(prime_length("MadaM") == true);
        assert!(prime_length("Wow") == true);
        assert!(prime_length("") == false);
        assert!(prime_length("HI") == true);
        assert!(prime_length("go") == true);
        assert!(prime_length("gogo") == false);
        assert!(prime_length("aaaaaaaaaaaaaaa") == false);
        assert!(prime_length("Madam") == true);
        assert!(prime_length("M") == false);
        assert!(prime_length("0") == false);
    }

}
