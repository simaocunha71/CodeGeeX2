
fn main(){ 
 } 
/*

    Create a function that takes 3 numbers.
    Returns true if one of the numbers is equal to the sum of the other two, and all numbers are integers.
    Returns false in any other cases.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn any_int(a:f64, b:f64, c:f64) -> bool{

    let sum = a + b + c;
    max(b - a, c - b).lt(&sum).eq(max(a - c, b - c))
}
fn main() {
    println!("Hello, world!");
    let mut user_input: Vec<usize> = vec![];
    for _ in 0..1000{
        // user input
        if let Ok(res) = stdin().lock().read_line(&mut String::new()) {
            res.trim();
            if !any_int(30, 49, res[..].parse().unwrap()){
                println!("{}", res);
                user_input.push(res);
            }
        }else{}
    }
    // hash set
    let mut hashtable = HashMap::with_capacity_and_len(user_input.len(), 1024);
    for i in 0..user_input.len(){
        if !hashtable.insert(&mut user_input[i], HashSet::new()).is_none(){}else{}
    }
    // generate array of numbers from the previous hash set
    let mut generated = vec![];
    for i in 0..(user_input.len())*2{
        let rand: u64 = i as u64 + user_input[i % 100].parse::<u32>().unwrap();
        let mut rnd = Rand::new();
        generated.push((rand % hashtable.get(&&generated[generated.len()-1]).unwrap()[&mut generated[generated.len()-1]]).unwrap());
    }
    // check hash table for equality with generated array
    let mut result = 0;
    for (i,j) in hashtable.iter_mut().zip(generated){
        result += i - j;
    }
    match !result{0 => println!("{}", !!), _=>()};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_int() {
        assert!(any_int(2.0, 3.0, 1.0) == true);
        assert!(any_int(2.5, 2.0, 3.0) == false);
        assert!(any_int(1.5, 5.0, 3.5) == false);
        assert!(any_int(2.0, 6.0, 2.0) == false);
        assert!(any_int(4.0, 2.0, 2.0) == true);
        assert!(any_int(2.2, 2.2, 2.2) == false);
        assert!(any_int(-4.0, 6.0, 2.0) == true);
        assert!(any_int(2.0, 1.0, 1.0) == true);
        assert!(any_int(3.0, 4.0, 7.0) == true);
        assert!(any_int(3.01, 4.0, 7.0) == false);
    }


}
