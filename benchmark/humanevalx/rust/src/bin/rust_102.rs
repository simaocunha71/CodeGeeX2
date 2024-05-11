
fn main(){ 
 } 
/*
This function takes two positive numbers x and y and returns the
    biggest even integer number that is in the range [x, y] inclusive. If 
    there's no such number, then the function should return -1.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn choose_num(x:i32, y:i32) -> i32{

    let x = x.max(0);
    if x != 0 {
        let mut biggest_even = 2; // we'll increase it by 1 each time a candidate is eliminated
        
        while (x+biggest_even) > y {
            let best = &mut big[best];
            let c = big.iter().take(3).min()?.0;
            
            if c < x-best && c >= 1 {
                replace(big, &mut best);
                
                for i in (x - best)..best + 2..{
                    if !big[i].is_none(){
                        big[i] = Some(max(choose_num((x-best+1).min(i), y));
                    }}
            }else {
                replace(big, &mut best);
            
                for i in (x - best)..best + 2..{
                    if !big[i].is_none(){
                        big[i] = Some((choose_num((i as isize)-(x-best), y));
                    }}
            }
            
            replace(&mut biggest_even, &(max(choose_num((x+1).min(biggest_even), y))); 
            replace(&mut best, &(max(choose_num((y-big[big][0] as i32).min(best), y)));  
        }
        
        return (x+biggest_even).min(y);
    }else{
        return (x + 1).min(y); // should work, but doesn't always. 
        //println!("{}", x);
     // println!("{} {}", x, y);
      //  println!("choose_num({},{})", x, y);
    }
}
struct Big{
    v:Vec<i32>
}
impl Big{
   /// the maximum number that we'll store in a given index.
   fn max(self) -> i32 {
        self.v[0] // 10,987
       .iter().map(|x| x.max(&mut self.v[0].len())).min()? + self.v

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_num() {
        assert!(choose_num(12, 15) == 14);
        assert!(choose_num(13, 12) == -1);
        assert!(choose_num(33, 12354) == 12354);
        assert!(choose_num(6, 29) == 28);
        assert!(choose_num(27, 10) == -1);
        assert!(choose_num(7, 7) == -1);
        assert!(choose_num(546, 546) == 546);
    }

}
