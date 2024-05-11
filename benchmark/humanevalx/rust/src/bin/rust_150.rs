
fn main(){ 
 } 
/*
A simple program which should return the value of x if n is 
    a prime number and should return the value of y otherwise.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn x_or_y(n: i32, x: i32, y: i32) -> i32 {

    if n % 2 == 0 {
        println!("prime number - {}", n);
        return x;
    } else {
        println!("non prime number {}", n);
        return y;
    }
}
fn main() {
    let mut p_list: Vec<Option<(i32, i32)>> = vec!();
    let mut p_set: HashSet<i32> = HashSet::new();
    for _ in 0..1000 {
        //println!("{}", rand::thread_rng().gen::<u8>());
        if rand::thread_rng() % 10 == 9 {
            println!("prime number - {}", rand::thread_rng().gen::<i32>() );
            p_list.push(Some((rand::thread_rng().gen::<i32>(), rand::thread_rng().gen::<i32>())));
            if p_set.insert(rand::thread_rng().gen::<i32>()) {
                println!("new prime found - {}", rand::thread_rng().gen::<i32>());
                replace(&mut p_list, 10); // 10 is the index of n in the set `p`
            } else {
                for _ in 0..rand::thread_rng() % rand::thread_rng().gen::<u8>() {// 10 is the index of n in the set `p`
                    replace(&mut p_set, rand::thread_rng() % rand::thread_rng().gen::<i32>()); // replace all the list elements with values in new set.
                } // end for loop
            } // end else (new prime found)
        } // end if (prime number)
    } // end for loop (1000 is the upper limit of the loop!)
    println!("prime numbers {}", rand::thread_rng().gen::<i32>()); // print the maximum element in the set p
    let x = max(p_list.iter() ).unwrap(); // get the max element from the list `x`
    for _ in

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_or_y() {
        assert_eq!(x_or_y(7, 34, 12), 34);
        assert_eq!(x_or_y(15, 8, 5), 5);
        assert_eq!(x_or_y(3, 33, 5212), 33);
        assert_eq!(x_or_y(1259, 3, 52), 3);
        assert_eq!(x_or_y(7919, -1, 12), -1);
        assert_eq!(x_or_y(3609, 1245, 583), 583);
        assert_eq!(x_or_y(91, 56, 129), 129);
        assert_eq!(x_or_y(6, 34, 1234), 1234);
        assert_eq!(x_or_y(1, 2, 0), 0);
        assert_eq!(x_or_y(2, 2, 0), 2);
    }

}
