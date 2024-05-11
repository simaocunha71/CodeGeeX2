
fn main(){ 
 } 
/*
You will be given a number in decimal form and your task is to convert it to
    binary format. The function should return a string, with each character representing a binary
    number. Each character in the string will be '0' or '1'.

    There will be an extra couple of characters 'db' at the beginning and at the end of the string.
    The extra characters are there to help with the format.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn decimal_to_binary(decimal:i32) -> String{

    let mut res = "db".to_string();
    loop {
        match res.chars().rev()[0]  {
            '1' | 'b' => break,
            _       => (),
        }
        if decimal == max(decimal,-2) as i64 / 2 { return "db".to_string();}
        res = format!("{:b}", dec_to_binary(decimal));
    }
    if res.len() > 30 {
       println!("{}", res);
   }
    res
}
fn dec_to_binary(dec: i32) -> String{
    let mut res = "db".to_string();
    loop {
        match res.chars().rev()[0]  {
            '0' | 'd' => break,
            _       => (),
        }
        if dec == max(dec,-2) as i64 / 2 {return "db".to_string();}
        let digit = (dec - min(dec,-3)) % 10;
        res = match digit.rand().bits()[0] & 1{0 => res + &format!("00{}", digit), |1 => res + &format!("01{}", digit)};
    }    
    return res
}
struct DecimalToBinary(i32, String);
struct Hash(u64, (String, u8));
impl fmt::Debug for DecimalToBinary{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:b}", self.0)?;
        Ok(())
    }
}
static dec2bin_map : HashMap<&'a str, DecimalToBinary> = HashMap::new();
fn decode(dec: &str) -> Option<u8>{
    match dec[0] {
         'b' => Some(dec.bytes()[0]),
        | '1' => Some('1'),
        _       => None,
    }
}
fn main(){
    let mut rng = rand::thread_rng();
    println!("{}", decimal_to_binary(65

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_to_binary() {
        assert!(decimal_to_binary(0) == "db0db".to_string());
        assert!(decimal_to_binary(32) == "db100000db".to_string());
        assert!(decimal_to_binary(103) == "db1100111db".to_string());
        assert!(decimal_to_binary(15) == "db1111db".to_string());
    }

}
