
fn main(){ 
 } 
/*
 Input is a space-delimited string of numberals from 'zero' to 'nine'.
    Valid choices are 'zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight' and 'nine'.
    Return the string with numbers sorted from smallest to largest
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn sort_numbers(numbers:String) -> String {

    let (mut hashmap, mut set):(HashMap<&str,Vec<usize>>, HashSet<usize>); 
    match numbers.as_ref() {
        Some(s) => hashmap = s.split(' ').map(|x| x.to_string()).collect(),
        None=>{}
    }
    for (key, val) in hashmap.iter_mut().map(|&(_,v)| (*v)) {
        let mut count=0;
        for num in val.into_iter() {
            if set.insert(*num).is_some(){break;}
            count+=1;
        }
        replace(&mut hashmap, key, count);
    }
    match hashmap.values().next().unwrap_or(Vec::new()).len() as usize {
        0 => "zero".to_owned(),
        _=>format!("{}", hashmap.values().next().unwrap_or(&"-".to_owned())),
    }
}
impl Any for TypeId<usize> {}
fn main() -> () {
    // Create a new Rng thread local to the current process:
    let rng = rand::thread_rng();
 
    println!("{:?}", max(sort_numbers("five".chars().map(|x| if x.is_alphabetic() { Some(x) } else { None }))));
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_sort_numbers() {
        assert!(sort_numbers("".to_string()) == "".to_string());
        assert!(sort_numbers("three".to_string()) == "three".to_string());
        assert!(sort_numbers("three five nine".to_string()) == "three five nine");
        assert!(
            sort_numbers("five zero four seven nine eight".to_string())
                == "zero four five seven eight nine".to_string()
        );
        assert!(
            sort_numbers("six five four three two one zero".to_string())
                == "zero one two three four five six".to_string()
        );
    }

}
