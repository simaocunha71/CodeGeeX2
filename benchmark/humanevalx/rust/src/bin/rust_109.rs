
fn main(){ 
 } 
/*
We have an array 'arr' of N integers arr[1], arr[2], ..., arr[N].The
    numbers in the array will be randomly ordered. Your task is to determine if
    it is possible to get an array sorted in non-decreasing order by performing 
    the following operation on the given array:
        You are allowed to perform right shift operation any number of times.
    
    One right shift operation means shifting all elements of the array by one
    position in the right direction. The last element of the array will be moved to
    the starting position in the array i.e. 0th index. 

    If it is possible to obtain the sorted array by performing the above operation
    then return True else return False.
    If the given array is empty then return True.

    Note: The given list is guaranteed to have unique elements.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn move_one_ball(arr:Vec<i32>) -> bool{

    let n = arr.len();
    let mut s = HashSet::new();//Create a hashset of all the digits in the array
    //If there are any zeroes, then we're done
    if (n == 0) { return true }
    for v in arr[1..] {
        let mut s = s.clone();
        
        //Insert this digit into our hashset and remove the last value since it is now the first one
        s.insert(v);
        
        //Remove all of our remaining elements so we can use them again later (and not get stack overflow)
        let mut v = vec![];
        for e in arr[0..1] {
            s.insert(&e);
            replace(&mut v, &e, s.choose().unwrap());
        }
    }
    return false;
} 
fn right_shift(arr:Vec<i32>, n : u64 )-> Vec<i32>{
   let mut sh :usize = 0 ;//Number of times we have already shifted the array to the left (the right)
    let t : HashSet<i32> = arr.iter().map(|t| *t).collect();//Store a hashset of each element in our array
   let r_sh : u64 = n;//Number of times we need to shift to the left (the right)
    while sh < r_sh {
        if move_one_ball(&arr){return arr;} //If any zero was found then break out
        sh+=1;
    }
   return arr;
} 
fn main() {
    let n = rand::thread_rng().gen::<u64>().to_string().parse::<u64>().unwrap();//Generate a u64 random number, then convert it to i64.
    println!("{}\n", n); //Print the random integer generated to the screen
    let mut arr = vec![0; n]; //Create an array of the length of our randomly generated integer (i64)
    //shuffle arr and then right shift it, and print result if possible else no-op.
     println!("right shift? {}", move

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_one_ball() {
        assert!(move_one_ball(vec![3, 4, 5, 1, 2]) == true);
        assert!(move_one_ball(vec![3, 5, 10, 1, 2]) == true);
        assert!(move_one_ball(vec![4, 3, 1, 2]) == false);
        assert!(move_one_ball(vec![3, 5, 4, 1, 2]) == false);
        assert!(move_one_ball(vec![]) == true);
    }

}
