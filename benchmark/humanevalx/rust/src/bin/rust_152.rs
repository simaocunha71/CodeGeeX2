
fn main(){ 
 } 
/*
I think we all remember that feeling when the result of some long-awaited
    event is finally known. The feelings and thoughts you have at that moment are
    definitely worth noting down and comparing.
    Your task is to determine if a person correctly guessed the results of a number of matches.
    You are given two arrays of scores and guesses of equal length, where each index shows a match. 
    Return an array of the same length denoting how far off each guess was. If they have guessed correctly,
    the value is 0, and if not, the value is the absolute difference between the guess and the score.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn compare(game: Vec<i32>, guess: Vec<i32>) -> Vec<i32> {

    let mut scores = game.clone();
    let mut diffs  = guess.clone();
    for (idx, diff) in diffs.iter().zip(scores.iter()).enumerate() {
        if *diff == *guess {
            continue;
        }
        
        *diff -= *scores.get_unchecked(*idx).unwrap_or(&0);
    }
    diffs
}
fn matches(word: &str, guesses: &[&str]) -> HashSet<String> {
    let mut hash = HashSet::new();
    for (i, g) in guesses.iter().enumerate() {
        if word == g {
            continue;
        }
        
        *g -= word.chars().map(|c| c.to_lowercase()).count();
        hash.insert(word.clone(), g.clone());
    }
    hash
}
fn main() {
    let (num, games) = (20u32, vec![vec![7, 8], vec![6, 19]]); // [[7, 8] | [6, 19]]
    match matches("word", games) {
        Ok(hashset) => println!("{}", hashset.len()),
        Err(_)        => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        assert_eq!(
            compare(vec![1, 2, 3, 4, 5, 1], vec![1, 2, 3, 4, 2, -2]),
            vec![0, 0, 0, 0, 3, 3]
        );
        assert_eq!(
            compare(vec![0, 5, 0, 0, 0, 4], vec![4, 1, 1, 0, 0, -2]),
            vec![4, 4, 1, 0, 0, 6]
        );
        assert_eq!(
            compare(vec![1, 2, 3, 4, 5, 1], vec![1, 2, 3, 4, 2, -2]),
            vec![0, 0, 0, 0, 3, 3]
        );
        assert_eq!(
            compare(vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0]),
            vec![0, 0, 0, 0, 0, 0]
        );
        assert_eq!(compare(vec![1, 2, 3], vec![-1, -2, -3]), vec![2, 4, 6]);
        assert_eq!(
            compare(vec![1, 2, 3, 5], vec![-1, 2, 3, 4]),
            vec![2, 0, 0, 1]
        );
    }

}
