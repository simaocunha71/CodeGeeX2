
fn main(){ 
 } 
/*
Given a positive integer N, return the total sum of its digits in binary.
    
    Variables:
        @N integer
             Constraints: 0 ≤ N ≤ 10000.
    Output:
         a string of binary number
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn solve(n:i32) -> String{

    let n = if n < 0 { n * (-1).abs() + 1 } else { n }; //make sure that n>=0 (it is a valid input!)
    let mut ret_str = String::new();
    //first, the leading zeroes of digits must be calculated
    let mut digits: Vec<u8> = vec![];
    while let Ok((i, v)) = n.to_digit() {
        for i in 0..v.digits().len() {
            if digits[digits.len() -1].unwrap() == 0 {
                digits.push(v.as_digit());
            } else {
                let mut j = 0;
                while let Ok((i, n)) = (j + 1).to_digit() {
                    if i >= 26 && !is_alpha_ascii(&n) && digits[digits.len() - 1].unwrap() == 0 {
                        digits.insert(i as u32, n);
                        break; //end of this inner for-loop (no need to find the next character)  
                    } else if i >= 26 && !is_alpha_ascii(&n) && digits[digits.len() -1].unwrap() != 0 {
                        digits[digits.len() - 1] = n; //when the current position of "i" is out of range, replace it with the digit (the next character in the alphabetical order) that follows the last digit found
                    } else if i >= 26 && let Ok(n) = digits[digits.len() - 1].to_digit() { //when "i" is within range: replace current position with digit found (before replacing, check whether a digit was found at this position).
                        digits[digits.len()-1] = n;
                    } else if i >= 26 && let Ok(n) = digits[0].to_digit() { //when "i" is within range: replace current position with the first digit
                        digits[digits.len() -1] = n;
                    } else { //all other positions
                        digits[j as usize]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert!(solve(1000) == "1");
        assert!(solve(150) == "110");
        assert!(solve(147) == "1100");
        assert!(solve(333) == "1001");
        assert!(solve(963) == "10010");
    }

}
