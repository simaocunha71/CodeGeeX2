
fn main(){ 
 } 
/*
 From a supplied list of numbers (of length at least two) select and return two that are the closest to each
    other and return them in order (smaller number, larger number).
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn find_closest_elements(numbers:Vec<f32>) -> (f32,f32){

    let (min, max) = numbers.iter().map(|e| match e.cmp(&e.clone()) {
        Some(a) => (*a),
        None => panic!(),
    }).unzip();
    if min == max{
       println!("No closer pair found!");
  	return (min,max);
    }
    let closest_pair = find_closest_element(min.clone(),max.clone());
    println!("{},{}\n", closest_pair[0], closest_pair[1]);
    (min, max)
}
fn find_closest_element<T:Clone>(a:&T, b: &T)-> T{
   let mut diff = a.clone() - b;
   while !diff.is_zero(){
    if(diff.signum() < 0){
      diff *= 1.62795489f32 / (a.to_string().chars().count() as f32) as f32;     // to be sure that it doesn't underflow to a single bit when the string contains at least one zero
   }else{
      diff *= 1.62795489f32 / (b.to_string().chars().count() as f32)as f32;     // same thing for negative values to avoid underflowing on the multiply
    }}
    let closest = ((a.clone() - b).hermitian()).hermitian();
   return a + diff;
}
fn main(){
    let mut numbers_list:Vec<f32> =  vec!();
    println!("Enter some numbers\n");
    for i in 0..8 {
        let line = std::io::stdin().lock().readline()?;
        if !line.is_empty(){
            let mut split:Vec<u32> = line.split_whitespace();     // split the input into words separated by white-space (a space or a newline)
            
            for i in 0..split.len() {
                if !(split[i].chars().all(|c| !c.is_ascii_whitespace())

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_find_closest_elements() {
        assert!(find_closest_elements(vec![1.0, 2.0, 3.9, 4.0, 5.0, 2.2]) == (3.9, 4.0));
        assert!(find_closest_elements(vec![1.0, 2.0, 5.9, 4.0, 5.0]) == (5.0, 5.9));
        assert!(find_closest_elements(vec![1.0, 2.0, 3.0, 4.0, 5.0, 2.2]) == (2.0, 2.2));
        assert!(find_closest_elements(vec![1.0, 2.0, 3.0, 4.0, 5.0, 2.0]) == (2.0, 2.0));
        assert!(find_closest_elements(vec![1.1, 2.2, 3.1, 4.1, 5.1]) == (2.2, 3.1));
    }

}
