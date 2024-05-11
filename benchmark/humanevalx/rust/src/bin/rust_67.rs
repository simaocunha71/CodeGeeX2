
fn main(){ 
 } 
/*

    In this task, you will be given a string that represents a number of apples and oranges 
    that are distributed in a basket of fruit this basket contains 
    apples, oranges, and mango fruits. Given the string that represents the total number of 
    the oranges and apples and an integer that represent the total number of the fruits 
    in the basket return the number of the mango fruits in the basket.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn fruit_distribution(s:&str, n:i32) -> i32 {

    let oranges = s.chars().filter(|c| c == 'O');
    let apples = s.chars().filter(|c| c == 'A');
    assert!(n > 0);
    let mut mango_fruit: i32 = n - oranges.len() - apples.len();
    if !apples.is_empty(){
        let idx = apples.iter().position(|c| c == 'A');
        
        for _ in 0..mango_fruit {
            mango_fruit -= oranges.len();
            
            replace(apples, idx, apples[idx] + 1);
        }    
    }
    mango_fruit
}
fn main() -> () {
    let s = "AAAOA";
    assert!(s == fruit_distribution(&s, 0)); // AAOA -> AAAO -> 3 -> 3
                                              // AAAO -> 4 -> 2
    let mango: i32 = 15;
    fn count(i:&str) -> i32 {fruit_distribution(i, mango)}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fruit_distribution() {
        assert!(fruit_distribution("5 apples and 6 oranges", 19) == 8);
        assert!(fruit_distribution("5 apples and 6 oranges", 21) == 10);
        assert!(fruit_distribution("0 apples and 1 oranges", 3) == 2);
        assert!(fruit_distribution("1 apples and 0 oranges", 3) == 2);
        assert!(fruit_distribution("2 apples and 3 oranges", 100) == 95);
        assert!(fruit_distribution("2 apples and 3 oranges", 5) == 0);
        assert!(fruit_distribution("1 apples and 100 oranges", 120) == 19);
    }


}
