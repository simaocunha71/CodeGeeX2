
fn main(){ 
 } 
/*
Create a function encrypt that takes a string as an argument and
    returns a string encrypted with the alphabet being rotated. 
    The alphabet should be rotated in a manner such that the letters 
    shift down by two multiplied to two places.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn encrypt(s:&str) -> String{

    let (x,y)=(0,1); 
        
    let mut res=String::from("");
     for ch in s.chars(){
        match(x-y){
            max => {res.push(ch.to_ascii_uppercase().unwrap())},
            _  =>{res.push(ch)},
        }
    }
    res
}
fn main() -> std::io::Result<()> {
     let rng:&Rng = &rand::thread_rng();
     // create a HashSet of each character
    let mut alphabet=HashSet::new(127);
     for ch in "abcdefghijklmnopqrstuvwxyz".chars(){
        alphabet.insert((alphabet.clone(),None));
     }
     // create a HashMap of each char with frequency
    let mut freq:HashMap<char,usize>=alphabet.len() as usize;
    for (&(key), &(ref)val ) in alphabet{freq[key]=freq.entry(*key).or_default_with(0);};
     // create a function to encrypt each character and store the encoded char as an index of a letter in the alphabet.
         let mut e:impl Fn(&str,&str) -> (usize,char) = |s1, s2| {e=|encrypt(s1); freq[e]-=s2.len()};
     
     // create a HashMap of letters with corresponding indexes.
    let mut map:HashMap<u8,usize>=freq.len() as usize;
    for (&index, &(ref)val ) in alphabet{map[index]=val};
     /*
        the next line creates an array representing every possible letter combo from 0 through N. 
         the array will contain strings such as "0123456789" and so on to "abcdefghijklmnopqrstuvwxyz".
         it then sorts this alphabeticaly by their values in map, so that for example if an A is
            inserted into the array, it will not come before another v in the list.
      */
    let mut a:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        assert!(encrypt("hi") == "lm");
        assert!(encrypt("asdfghjkl") == "ewhjklnop");
        assert!(encrypt("gf") == "kj");
        assert!(encrypt("et") == "ix");
        assert!(encrypt("faewfawefaewg") == "jeiajeaijeiak");
        assert!(encrypt("hellomyfriend") == "lippsqcjvmirh");
        assert!(
            encrypt("dxzdlmnilfuhmilufhlihufnmlimnufhlimnufhfucufh")
                == "hbdhpqrmpjylqmpyjlpmlyjrqpmqryjlpmqryjljygyjl"
        );
        assert!(encrypt("a") == "e");
    }

}
