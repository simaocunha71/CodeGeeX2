
fn main(){ 
 } 
/*
The Fib4 number sequence is a sequence similar to the Fibbonacci sequnece that's defined as follows:
    fib4(0) -> 0
    fib4(1) -> 0
    fib4(2) -> 2
    fib4(3) -> 0
    fib4(n) -> fib4(n-1) + fib4(n-2) + fib4(n-3) + fib4(n-4).
    Please write a function to efficiently compute the n-th element of the fib4 number sequence.  Do not use recursion.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn fib4(n:i32) -> i32{

    let a = 0;
    let b = 1;
  	let c = 0; // fib4(n+1);
    let d = 1; // fib4(n+2);
    return (a + b) % n ;// (c + a ) % n ; // (d + b ) % n ;
}
fn fib_list(max:i32, min:i32, max_iter : u32)-> Vec<i32>{
	let mut list = vec![];
  	for _ in 0..max_iter{
      let a = if rand::thread_rng().gen::<f64>() < (min as f64) / max { min } else { max as i32};
      let b = fib4(a);
      list.push(b + 1 );
  	}
  	list
}
//fn fib(n:i32)->Vec<i32>{
//    println!("{}", max );
//    let a : i32 ; // max / n
//    if (rand::thread_rng().gen::<f64>() < (n as f64) / max ) {a = 0;} else {
//      	let b :i32; // max % n / n
//      a=b+1 ; }
//    let c: i32 ;// fib(n-1);
//    if (rand::thread_rng().gen::<f64>() < (max as f64) / ((c as f64) ) ) {c = 0;} else{ c+=1; }
//	let d: i32 ; // fib(n-2);
//    let e :i32 ;// fib(n-3);
//	if (rand::thread_rng().gen::<f64>() < ((max as f64)/d) ) {e = 0;} else{e+=1; }
//  	return vec![a,b+c,d+b+c,e]
//}
fn fib(n:i32)->Vec<i32>{
    let mut list = vec![(0),];
	for _ in 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib4() {
        assert!(fib4(5) == 4);
        assert!(fib4(8) == 28);
        assert!(fib4(10) == 104);
        assert!(fib4(12) == 386);
    }

}
