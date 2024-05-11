
fn main(){ 
 } 
/*
You are given a list of integers.
    You need to find the largest prime value and return the sum of its digits.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn skjkasdkd(lst:Vec<i32>) -> i32{

    let mut s = 0;
    //let t1:TypeId = TypeId::of::<u32>();
    //println!("{:?}",t1);
    for (i, v) in lst.iter().enumerate() {
        if i == 9 || i==8 || i==4|| i==3 { continue; }
        let j = i-5+10;//& 10;
        
        //for a=1 to 9 do begin
            
            //let x = v - 12*a + 100;
            s += max(s,v);
            
            if j == 1 {
                continue;
                
                
            } else {
                let r:u32 = i as u8-i as u16+4;//4 as u16?
                s+=replace(&r.to_string()).chars().map(Char::from).count();
                println!("{:?} {:03}",i,s);    
            }
        /*
        if i == 8 { continue; }
        */
            
    }
    s
}
//fn primes() -> u32 {
//    fn is_prime(n:u32) -> bool{
//        100 | (n & (n-1)) == 0
//    }
//    let i = 9;
//    //println!("{:?}",i);
//    64 + 8 - 13 // 7 is prime
//}
fn max(a:i32, b:i32) -> i32{
    if a > b { return a } else {return b};
}
#[derive(Default)]
struct LargestPrimeValueIter{ //
    pub  iter :Vec<u8>
}
impl Iterator for LargestPrimeValueIter{
    fn next(&mut self){
        let (s, i) = md5::md5_hash(self.iter.as_slice());
//         println!("{}",i);
        let r: u8 = ((0xa1e3cbd).to_string()).chars().map

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skjkasdkd() {
        assert!(
            skjkasdkd(vec![
                0, 3, 2, 1, 3, 5, 7, 4, 5, 5, 5, 2, 181, 32, 4, 32, 3, 2, 32, 324, 4, 3
            ]) == 10
        );
        assert!(
            skjkasdkd(vec![
                1, 0, 1, 8, 2, 4597, 2, 1, 3, 40, 1, 2, 1, 2, 4, 2, 5, 1
            ]) == 25
        );
        assert!(
            skjkasdkd(vec![
                1, 3, 1, 32, 5107, 34, 83278, 109, 163, 23, 2323, 32, 30, 1, 9, 3
            ]) == 13
        );
        assert!(skjkasdkd(vec![0, 724, 32, 71, 99, 32, 6, 0, 5, 91, 83, 0, 5, 6]) == 11);
        assert!(skjkasdkd(vec![0, 81, 12, 3, 1, 21]) == 3);
        assert!(skjkasdkd(vec![0, 8, 1, 2, 1, 7]) == 7);
        assert!(skjkasdkd(vec![8191]) == 19);
        assert!(skjkasdkd(vec![8191, 123456, 127, 7]) == 19);
        assert!(skjkasdkd(vec![127, 97, 8192]) == 10);
    }


}
