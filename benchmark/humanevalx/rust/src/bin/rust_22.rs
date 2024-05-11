
fn main(){ 
 } 
/*
 Filter given list of any python values only for integers
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn filter_integers(values: Vec<Box<dyn Any>>) -> Vec<i32> {

    let hashmap = HashMap::new();
    for (v, _, _) in values.iter() {
        hashmap.insert(*v.as_ref(), 0);
    }
    for ((k, v), count) in hashmap.iter() {
        *count += if let Some(i: i32) = hashmap.remove(&k) {
            if let Ok(Some(i)) = try!(ascii_str::parse_as_i32(replace(*v, k))) {
                i
            } else {
                0
            }
        } else { 0 };
    }
    hashmap.values()
}
fn generate_random_integers(count: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut set = HashSet::new(&mut rng);
    let mut result: Vec<i32> = Vec::with_capacity(4000);
    while count > 0 && !result.is_empty() {
        match &mut set.drain(1).take(min((count / 16) as usize, rng.gen::<usize>())) {
            v => result.push(*v % 257),
            
        }
    }    
    // let mut hashmap = HashMap::new();
    // for integer in set.drain(..) {
    //      *hashmap.insert(integer, 1) += 1;
    // }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_integers() -> (){
        let values = vec![
                        &"0",
                       &"1",
                       &"2",
                       &"3",
                       &"4",
                       &"5",
                       &"6",
                       &"7",
                       &"8",
                       &"9",
                       &"A",
                        &"B",
                        &"C",
                        &"D",
                

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_filter_integers() {
        assert_eq!(filter_integers(vec![]), vec![]);
        let v_empty: Vec<Box<dyn Any>> = vec![];
        assert_eq!(
            filter_integers(vec![
                Box::new(4),
                Box::new(v_empty),
                Box::new(23.2),
                Box::new(9),
                Box::new(String::from("adasd"))
            ]),
            vec![4, 9]
        );
        assert_eq!(
            filter_integers(vec![
                Box::new(3),
                Box::new('c'),
                Box::new(3),
                Box::new(3),
                Box::new('a'),
                Box::new('b')
            ]),
            vec![3, 3, 3]
        );
    }

}
