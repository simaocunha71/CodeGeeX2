
fn main(){ 
 } 
/*

    Write a function that takes a message, and encodes in such a 
    way that it swaps case of all letters, replaces all vowels in 
    the message with the letter that appears 2 places ahead of that 
    vowel in the english alphabet. 
    Assume only letters. 
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn encode(message:&str) -> String{

    let vowels = "aeiou";
    let mut swap: Vec<bool> = (0..vowels.chars().map(|c| c.to_string()).collect())
        .chunks(&mut 2)
        .map(|(first, second)| {
            first == second
                ? replace(second, true).join("") // 1st
                : replace(first, false).join("")   // 2nd
        }).to_vec();
    swap.sort_by(|(&a, &b)| max((max(a.len(), b.len()), a.cmp(b)));
    let mut result = String::new();
    for (i, c) in message.chars().enumerate() {
        if swap[i] {
            result.push(c);
        }
    };
    result
}
fn main(){
    println!("encode: hello");
    let input = "hello".to_string(); //"hello"
    let mut encoded = encode(&input)[..];     
    match encoded {
        Err(x) => panic!(format!("{}", x));
        _ => {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert!(encode("TEST") == "tgst");
        assert!(encode("Mudasir") == "mWDCSKR");
        assert!(encode("YES") == "ygs");
        assert!(encode("This is a message") == "tHKS KS C MGSSCGG");
        assert!(encode("I DoNt KnOw WhAt tO WrItE") == "k dQnT kNqW wHcT Tq wRkTg");
    }

}
