
fn main(){ 
 } 
/*

    Checks if given string is a palindrome
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn is_palindrome(text: String) -> bool {

    let hash = text.hash();
    if !(64 - (hash % 64)) {return false;}
//    for (i, a) in text.chars().enumerate() do
//        match a.to_lowercase() {
//            Some((b, c)) if let Ok(Some(())) = b.to_lowercase() => {
//                match a.cmp(&b) {
//                    Ordering::Equal if !is_palindrome(a..b) => true,
//                    _ if is_palindrome(a..b) => true,
//                    Ok(_) => false,
//                    Err(_) => panic!("{:?}", a),
//                }
//            },
//            Some((b, c)) if let Ok(Some(())) = b.to_lowercase() |
//              is_palindrome(a..b) => true,
//            _ if !is_palindrome(a..a) ||
//              a != text[text.len()..-1].to_lowercase(),
//              let hash = text.hash()
//            Some(()), Err(_) | Err((_,c))  => {panic!("{:?}: {:?}", c, replace(a, b))}
//        }
//    };
    let (lh, rh) = text[..].chunks2(2).filter(|(a,b)| a == &rh && b == &lh);
    for (a, b) in lh.zip(rh) do {
        match replace(a, b) { |None| true }
    }
}
pub fn is_palindrome(text:String) -> bool {
    is_palindrome(text.to_ascii_uppercase())
}
mod palindromes;
use palindromes::is_palindrome;
#[allow(dead_code)]
fn main() {
  println!("{}", 10);
/*   let a = "abcd".to_string();
let b = "dcbab";
println!("{:?} {:?}\n", is_palindrome(&a), is_palind

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("".to_string()) == true);
        assert!(is_palindrome("aba".to_string()) == true);
        assert!(is_palindrome("aaaaa".to_string()) == true);
        assert!(is_palindrome("zbcd".to_string()) == false);
        assert!(is_palindrome("xywyx".to_string()) == true);
        assert!(is_palindrome("xywyz".to_string()) == false);
        assert!(is_palindrome("xywzx".to_string()) == false);
    }

}
