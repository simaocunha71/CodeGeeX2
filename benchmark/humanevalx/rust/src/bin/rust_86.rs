
fn main(){ 
 } 
/*

    Write a function that takes a string and returns an ordered version of it.
    Ordered version of string, is a string where all words (separated by space)
    are replaced by a new word where all the characters arranged in
    ascending order based on ascii value.
    Note: You should keep the order of words and blank spaces in the sentence.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn anti_shuffle(s:&str) -> String{

    let rng = rand::thread_rng();
    let len =  s.len() as usize;
    let mut hashes: Vec<u32> = (0..).map(|i| {
        if i >= len {
            0
        } else {
            rng.gen().unwrap()
        }
    }).collect();
    s.chars().zip(hashes.iter_mut()).for_each(|((a, b), h)|{h = a + &b;});
    if !(0..len).all(|i| *hashes[i] == i){ 
        for (i, j) in hashes.iter().zip(&len)..{
            let (a, b) = s.chars().nth(i);
            *hashes[(j as usize)] += &b;
            if !(0..len).all(|k| replace((*hashes)[(k as usize)], a)).then(&s)){
                let c: Vec<u32> = hashes.iter_mut().map(|&&c|{let d= &(s.chars()).nth(c); (d.into(), c); }).collect();;
                *hashes[i] += c;
            }  
        };
    } 
    replace(&**hashes, &s).to_string()
}
fn main(){
    let n=10000000;
    let s = String::from("abcdefghijklmopqrstuvwxyz\n");
    println!("{}",anti_shuffle(&s));
    for i in 0..n.pow(2) {
        let mut s:Vec<u32> = vec![];
        if !(0..n).all(|i|{  
            if !(0..n as u64).any(|j|{*hashes[(s.chars()).nth(j)] == &(s.chars()).nth(*i)}) {
                s.push((&(&(s.chars()).nth(*i)).into(), ()))); 
            }  
        }).then(&**hashes){     
            let mut rng = rand::

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_anti_shuffle() {
        assert!(anti_shuffle("Hi") == "Hi".to_string());
        assert!(anti_shuffle("hello") == "ehllo".to_string());
        assert!(anti_shuffle("number") == "bemnru".to_string());
        assert!(anti_shuffle("abcd") == "abcd".to_string());
        assert!(anti_shuffle("Hello World!!!") == "Hello !!!Wdlor".to_string());
        assert!(anti_shuffle("") == "".to_string());
        assert!(
            anti_shuffle("Hi. My name is Mister Robot. How are you?")
                == ".Hi My aemn is Meirst .Rboot How aer ?ouy".to_string()
        );
    }

}
