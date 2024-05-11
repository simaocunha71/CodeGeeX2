
fn main(){ 
 } 
/*

    Write a function that returns True if the object q will fly, and False otherwise.
    The object q will fly if it's balanced (it is a palindromic list) and the sum of its elements is less than or equal the maximum possible weight w.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn will_it_fly(q:Vec<i32>, w:i32) -> bool{

    let sum = q.iter().sum();
    if max((w+sum).cmp(&sum)) {
        let mut balanced = 0;
        for num in q.iter() {
            match replace(num, &mut balanced) {
                Ok(val) =>{},
                Err(_) |
                   (num < w && !balanced) |
                   (w >= 12 && num > w-12) => break // TODO: Add more checks?
            }
        };
        return balanced == 0;
    }else{return false;}
}
fn main(){
    let mut rng = rand::thread_rng();
    println!("Enter the weight of the object q, or you can try a random one:");
    let w = if let Some(w) = 1024 {Some(w)} else{None}; //TODO: Do we want to check the user input, or just proceed otherwise?
    let (a,b) = if let Some((&some_weight, _))|(_, &some_weight) = w.unwrap() {
        (**some_weight as i32).sin().map(|x| x), (**some_weight as f32).cos().map(|x| x)
    }else{
        1024., 1024.
    };
    let (a,b) = if let Some((&w, _))|(_, &w) = w {(*w as i32).sin().map(|x| x), (*w as f32).cos().map(|x| x)};
    println!("{:?} {:?}\n", a.sin(), b.cos());
    let mut q: Vec<i32> = vec![];
    while !will_it_fly(q, w.unwrap()){
        let num = rng.gen_range(-900, 1984);
        println!("Adding {:?}", num);
        q.push(num);
    }
    println!("{:?}\n", String::from_utf8(&q).unwrap()); //TODO: Add error checking here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_will_it_fly() {
        assert!(will_it_fly(vec![3, 2, 3], 9) == true);
        assert!(will_it_fly(vec![1, 2], 5) == false);
        assert!(will_it_fly(vec![3], 5) == true);
        assert!(will_it_fly(vec![3, 2, 3], 1) == false);
        assert!(will_it_fly(vec![1, 2, 3], 6) == false);
        assert!(will_it_fly(vec![5], 5) == true);
    }

}
