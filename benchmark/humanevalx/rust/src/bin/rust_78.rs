
fn main(){ 
 } 
/*
You have been tasked to write a function that receives 
    a hexadecimal number as a string and counts the number of hexadecimal 
    digits that are primes (prime number, or a prime, is a natural number 
    greater than 1 that is not a product of two smaller natural numbers).
    Hexadecimal digits are 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C, D, E, F.
    Prime numbers are 2, 3, 5, 7, 11, 13, 17,...
    So you have to determine a number of the following digits: 2, 3, 5, 7, 
    B (=decimal 11), D (=decimal 13).
    Note: you may assume the input is always correct or empty string, 
    and symbols A,B,C,D,E,F are always uppercase.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn hex_key(num:&str) -> i32{

    let s = num.chars().collect::<Vec<char>>();
    var len:i32=s.len();
    if (10 & (len as i32)) != 0 {
        return -1;
    }
    // 0b
    let mut index_num = s.iter().position(|&c| c == '0' || c == 'b').unwrap_or(0);
    if (index_num-8) > len as i32 {
        return -1;
    } else {
        index_num += 8; // 8-index
    }
    // 0x, 0X
    let mut index_char = s.iter().position(|&c| c == '0' || c == 'x').unwrap_or(0);
    if (index_char <= index_num && index_char > len as i32) {
        return -1;
    } else {
        index_num += index_char-1; // 1-index
    }
    for _ in 0..8{
        let mut index:i32=&index_num.saturating_add(8);
        
        let ch = s.chars().nth(index & 0xFF).unwrap();
        if (is_prime_number(ch)) {
            // 11
            len -= 8; index += 8-1;
            break;
        } else {
            index += 8-1;
        }
    }
    return index as i32;
}
#[derive(Debug, PartialEq)]
enum TypeOfPrime{
    Number, B, D, E, F
}
impl std::fmt::Display for TypeOfPrime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self.try_into().unwrap();
        match self{ 
            Number => write!(f, "{}", ) + "Number"),
            B => write!(f, "b") + "B"),
            D => write!(f, "d") + "D"),
            E => write!(f, "e")

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_hex_key() {
        assert!(hex_key("AB") == 1);
        assert!(hex_key("1077E") == 2);
        assert!(hex_key("ABED1A33") == 4);
        assert!(hex_key("2020") == 2);
        assert!(hex_key("123456789ABCDEF0") == 6);
        assert!(hex_key("112233445566778899AABBCCDDEEFF00") == 12);
        assert!(hex_key("") == 0);
    }

}
