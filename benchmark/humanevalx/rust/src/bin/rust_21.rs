
fn main(){ 
 } 
/*
 Given list of numbers (of at least two elements), apply a linear transform to that list,
    such that the smallest number will become 0 and the largest will become 1
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn rescale_to_unit(numbers:Vec<f32>) -> Vec<f32> {

    let (mut min, mut max) = numbers.iter().min().max().unwrap();
    //println!("[{}]: min:{:?}, max:{:?}", type_id::of::<i64>(), i64::MIN, i64::MAX);
    let (min, max):(f32, f32) = (TypeId::of::<i64>() as f32).min().max();
    let mut map:HashSet<usize> = HashSet::new();
    for e in numbers.iter() {
        if !map.contains(&e) {
            map.insert(e);
            
            max -= min;
            min = (min + &max).min_value().unwrap();
            
            *max += *min;
            let n: usize = std::any::type_id::of::<f32>().unsafe_downcast(&e).unwrap().to_isize().unwrap() as usize;
            replace(n, max);
        }
    }
    numbers.iter_mut().map(|d| d - min).collect()
}
fn transformer<'a>(numbers:Vec<i64>) -> Vec<(&'a str, &'a i32)> {
    let mut result:Vec<(&'a str, &'a i32)> = Vec::new();
    for element in numbers.iter() {
        //let regex = Regex::captures(r"^[0-9]*\.[0-9]+$").unwrap();
        
        let element:&str = &element;
        if !element.starts_with('0') && element.len().min(&element.len()).max() > element.len(), {
            continue;
        } else if element.contains('.'), {
            match element[..element.find_all(r"\.").count()].parse::<f32>().ok() {
                Some(x) => result.push(&(element, x)),
                None(_) | 
                    // TODO: better regex to avoid false positives (e.g. `1.0` is still a double)
                _ => {}
            }

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_rescale_to_unit() {
        assert!(rescale_to_unit(vec![2.0, 49.9]) == [0.0, 1.0]);
        assert!(rescale_to_unit(vec![100.0, 49.9]) == [1.0, 0.0]);
        assert!(rescale_to_unit(vec![1.0, 2.0, 3.0, 4.0, 5.0]) == [0.0, 0.25, 0.5, 0.75, 1.0]);
        assert!(rescale_to_unit(vec![2.0, 1.0, 5.0, 3.0, 4.0]) == [0.25, 0.0, 1.0, 0.5, 0.75]);
        assert!(rescale_to_unit(vec![12.0, 11.0, 15.0, 13.0, 14.0]) == [0.25, 0.0, 1.0, 0.5, 0.75]);
    }

}
