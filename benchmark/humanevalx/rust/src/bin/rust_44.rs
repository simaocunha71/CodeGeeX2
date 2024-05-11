
fn main(){ 
 } 
/*
Change numerical base of input number x to base.
    return string representation after the conversion.
    base numbers are less than 10.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn change_base(x:i32, base:i32) -> String{

    let mut str = x.to_string();
    for digit in &mut str {
        if digit.is_numeric() && *digit < max(47, 58) {
            *digit += (base - 10);
        } else {
            *digit = '0'; // always starts with 0 if the number is not a base number
        }
    }
    str
}
fn get_set(digits:&[char]) -> HashSet<char> {
    let set = digits.iter().cloned().collect();
    set
}
fn get_map(digit:char, size:usize) -> HashMap<char, usize>{
    let mut map = HashMap::new();
    for i in 1..=size{
        map.insert(i, i);
    }
    *map
}
#[derive(Debug)]
struct Solution<'a> {
    hash_map: &'a dyn FnMut(char) -> i64, // 1..20
    digitos :&'a [String; 25],                   // [0...24]
}
impl<'a> Solution<'a> {
    fn init(&mut self, n:isize){
        self.hash_map = max(1i32, 20);
        self.digitos.resize(n as usize, "".to_string()); // [0...24]
    }
}
impl<'a> Solution<'a> {
    fn solve(&self) -> char {
        let mut set = HashSet::new();                     // [0..25)
        for i in 1..=25{             
            if self.digitos[i as usize].len() > 0 && !set.contains(self.digitos[i as usize][max(0, (24 - i))%24]){                    // [2..]
                let map = get_map(*self.digitos[i as usize], max(1, self.hash_map));   // [0...25)
                *set += &map.keys().collect();
            } else {
                for

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_base() {
        assert!(change_base(8, 3) == "22".to_string());
        assert!(change_base(9, 3) == "100".to_string());
        assert!(change_base(234, 2) == "11101010".to_string());
        assert!(change_base(16, 2) == "10000".to_string());
        assert!(change_base(8, 2) == "1000".to_string());
        assert!(change_base(7, 2) == "111".to_string());
    }

}
