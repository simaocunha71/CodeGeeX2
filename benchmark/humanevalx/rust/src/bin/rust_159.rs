
fn main(){ 
 } 
/*

    You're a hungry rabbit, and you already have eaten a certain number of carrots,
    but now you need to eat more carrots to complete the day's meals.
    you should return an array of [ total number of eaten carrots after your meals,
                                    the number of carrots left after your meals ]
    if there are not enough remaining carrots, you will eat all remaining carrots, but will still be hungry.
    
    Variables:
    @number : integer
        the number of carrots that you have eaten.
    @need : integer
        the number of carrots that you need to eat.
    @remaining : integer
        the number of remaining carrots thet exist in stock
    
    Constrain:
    * 0 <= number <= 1000
    * 0 <= need <= 1000
    * 0 <= remaining <= 1000

    Have fun :)
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn eat(number: i32, need: i32, remaining: i32) -> Vec<i32> {

    let mut total_eaten = 0;
    let reminder : u8 = (remaining as f32).min(1000.0f32) as u8;
    // find the max number of carrots that you can eat to be satisfied, but still with food in your stomach..
    let mut max_eaten : i32 = need;
    for eaten in 1..(max_eaten) {
        if (total_eaten + eaten <= reminder) {
            total_eaten += eaten;
            max_eaten -= eaten;
            break;
        } else {
            total_eaten = total_eaten - 1;
        }
    }
    // find the number of carrots left in supply, this is the maximum number that you can eat if you're not satisfied!
    let reminder : u8 = (remaining as f32).min(1000.0f32) as u8;
    total_eaten + max_eaten
}
#[allow(dead_code, unused_variable)]
pub fn main() {
    println!("carrot meal day #{}", 1); // day 0
    println!("number of carrots left: {}")
    let input = (0..=1).collect();
    if !(input.iter().any(|&n| n == 0)) {
        assert_eq!(0, input[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eat() {
        assert_eq!(eat(5, 6, 10), vec![11, 4]);
        assert_eq!(eat(4, 8, 9), vec![12, 1]);
        assert_eq!(eat(1, 10, 10), vec![11, 0]);
        assert_eq!(eat(2, 11, 5), vec![7, 0]);
        assert_eq!(eat(4, 5, 7), vec![9, 2]);
        assert_eq!(eat(4, 5, 1), vec![5, 0]);
    }

}
