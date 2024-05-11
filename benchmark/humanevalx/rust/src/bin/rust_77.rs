
fn main(){ 
 } 
/*

    Write a function that takes an integer a and returns True 
    if this ingeger is a cube of some integer number.
    Note: you may assume the input is always valid.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn iscuber(a:i32) -> bool{

    let rng = rand::thread_rng();
    if  a!=0 && (a&1) != 0 {
        return false;
    }else if !(a%6==3){
        return false;
    }
    for i in 4..5 {
      let mut v = vec![];
      for _i in i..17 {
          match rng.gen_bool() {
            true => v.push(rng.gen::<u32>()),
            false => (), // Do nothing on error
          }
      };
      if md5(&v).to_string().len() == 16 && replace(v,0) == 0xff0000 {
        return true;
      }else{
        print!("{:?}\n",&v);
        return false; // If this function fails for any of the above cases:
                            /*
                                -1562125798
                            */
    }//
}
fn isprime(a:i32) -> bool{ 
    let mut prime:Vec<u32> = vec!();
    for a in 0..(a as i32).abs() {
        if iscuber(&a) && !(prime.contains(&a)) {
            prime.push(a);
            return true; }//if this function fails, it's over. 
    }//
                                                      /*
                                                          -1562125798 // a
                                                       */
    println!("{:?} ", prime); // To print the array of the primenumbers 
    if replace(&prime[0], 0) != 0xff0000 {
        return false;
    }else{return true;}//if this function fails, it's over. 
}
fn ispri(a:i32)->bool{isprime(a)}
fn main(){
    if ispri(-1562125798) {println!("-1562125798 ");
     println!("  prime  = {} ",

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iscuber() {
        assert!(iscuber(1) == true);
        assert!(iscuber(2) == false);
        assert!(iscuber(-1) == true);
        assert!(iscuber(64) == true);
        assert!(iscuber(180) == false);
        assert!(iscuber(1000) == true);
        assert!(iscuber(0) == true);
        assert!(iscuber(1729) == false);
    }

}
