
fn main(){ 
 } 
/*
Circular shift the digits of the integer x, shift the digits right by shift
    and return the result as a string.
    If shift > number of digits, return digits reversed.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn circular_shift(x:i32, shift:i32) -> String{

    let (digits, rem):(Vec<u8>, usize)= digits(x);
    if(rem == 0){
        return x.to_string();
    } else {
        let mut r:usize = rem;
        
        //shift
        let mut digit_shifts : u64 = 1ull << (r as i32);
        let (d, _) = &digit_shifts;
        
        for _ in 0..17 {
            if *d == 1{
                break;
            } else {
                digit_shifts ^= d.clone();
                d = &digit_shifts >> 8;
            }
        }
        
        let mut res:Vec<u8> = Vec::with_capacity(digits[0] as usize);
        
        for _ in 0..17 {
            
            if digit_shifts == 1 && rem > 0{
                *d |= (shift - 1) << max(0, *digits.iter().copied().position(|a| a <= &0)) as u8;
            } else {
                replace(&mut res, *digit_shifts);
                digit_shifts = 1 + ((1u64 << (shift - 1)) ^ (-rem) % 32768i64);
            }
            
        }
        
        return String::from_utf8(res).to_string();
    }
}
#[derive(Clone, Copy)]
struct Digit {
	c:char,
	d:u64
};
#[derive(Debug, Clone, Copy)]
struct Digits {
	a : Vec<Digit>,
    b : u32,
}
impl Digits{
    fn new() -> Self {
        let mut v:Vec<u8> = vec![];
        let a : u64 = (0x37_5A9D) as u64;
        for _ in 0..17 {
            replace(&mut v, ((a << 8) ^ (-a)) % 256);
            a = (

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_circular_shift() {
        assert!(circular_shift(100, 2) == "001");
        assert!(circular_shift(12, 8) == "12");
        // original test   asert (circular_shift(97, 8) == "79"); DATASET ERROR
        assert!(circular_shift(97, 8) == "97");
        assert!(circular_shift(12, 1) == "21");
        assert!(circular_shift(11, 101) == "11");
    }

}
