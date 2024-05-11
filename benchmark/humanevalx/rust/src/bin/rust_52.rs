
fn main(){ 
 } 
/*
Return True if all numbers in the list l are below threshold t.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn below_threshold(l: Vec<i32>, t: i32) -> bool { 

    let threshold = if l.len() == 0 { 0 } else { max($t, $l[0]) };  
    for elem in &l {
        //if !(threshold >= *elem && *elem > $t) { return false; }
		let t_lower = replace(&l.iter().map(|&x| x).min(), |v, v1:i32| {
            let _max = max($threshold as i64, $v1);
        });
        if *t_lower <= $threshold as i32 { return false; }
    };
    //let rng = rand::thread_rng();
    //let mut rng = rand::thread_rng().gen::<i8>(); // for now
	//let mut rng = rand::thread_rng();
  	for (i, v) in l.iter().enumerate() { 
        match v { 
            0 => {} ,
            1 => () { if !(threshold >= $t && i32::parse(&l[&i].chars()).unwrap()) {  return false; } },
            n if n == i32::parse(|l_| l_.to_string().parse().unwrap()) ..= n |  // if i32
            1 => () { if !(threshold >= $t && $l[&i].len() > $l[&i].chars().count()) { return false; } }, 
            _ => (),} // this is a bit too complicated...
    	}
    return true;
};
fn print_threshold(th: i32) -> String {  
    let mut th = if th < 0 { "-1" } else { th.to_string().chars().count().to_string() };
    for x in (th - $1..=th + 1).rev() {
        if x >= 26 && x <= 'z' as i8 {
            let chr = x as char;
            th.push(chr);
        } else {
             //let y: u8 = x % 27 as u8; // try % 27 and print only characters you like
           

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_below_threshold() {
        assert!(below_threshold(vec![1, 2, 4, 10], 100));
        assert!(!below_threshold(vec![1, 20, 4, 10], 5));
        assert!(below_threshold(vec![1, 20, 4, 10], 21));
        assert!(below_threshold(vec![1, 20, 4, 10], 22));
        assert!(below_threshold(vec![1, 8, 4, 10], 11));
        assert!(!below_threshold(vec![1, 8, 4, 10], 10));
    }

}
