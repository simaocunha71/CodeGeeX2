
fn main(){ 
 } 
/*

    Given two lists operator, and operand. The first list has basic algebra operations, and 
    the second list is a list of integers. Use the two given lists to build the algebric 
    expression and return the evaluation of this expression.

    The basic algebra operations:
    Addition ( + ) 
    Subtraction ( - ) 
    Multiplication ( * ) 
    Floor division ( // ) 
    Exponentiation ( ** ) 

    Note:
        The length of operator list is equal to the length of operand list minus one.
        Operand is a list of of non-negative integers.
        Operator list has at least one operator, and operand list has at least two operands.

    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn do_algebra(operato: Vec<&str>, operand: Vec<i32>) -> i32 {

    let len = operato.len();
    if len == 1 && contains_int(&operato[0].to_ascii_ref()){ // one operator, single integer
        return operato[0].parse().unwrap();
    }
    let mut accumulator: Vec<i32> = vec![]; // build an intermediate list.
    for (op, i) in operato.iter_mut().enumerate() { // modify the intermediate list to hold a combination of operators and/or numbers
        match op.as_str(){
            "+" => accumulator.push(i), // add the integer
            "-" => if let Some(number) = i32::parse(&operand[i]) {accumulator.push(-1 * number)} else {}, 
            
        }
    }
    let (op, i) = accumulator[len-1].to_ascii().splitn(2, ' ') // parse out the operators and the operands
           .iter()// map the integers to their binary representations, and use ascii characters to represent the binaries, then
           .cloned().map(|(x, y)| (y.parse::<f64>(), x))
           .collect(); // group by element type
    match (op[0].as_str(), op[1].as_str()){ 
        ("+", "+") | ("-", "-") => {
            let res = (op[0] as f64) + (op[1] as f64);  // apply the operators to their binary representation, convert back to integers, and add them
            return res.parse().unwrap();
        },
         ("+", "-") | ("-", "+") => {
             let res = if let Some(number) = i32::parse(&operand[i]){ number + (op[0] as f64)} else {};
            return res;  // apply the operator to the binary representation, convert back to integers and add them.
        },
         ("*", "*") | ("/", "/") => { 
             let mut accumulator = (op[0] as f64) * (op[1] as f64); // multiply by the operator and its binary

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_algebra() {
        assert_eq!(do_algebra(vec!["**", "*", "+"], vec![2, 3, 4, 5]), 37);
        assert_eq!(do_algebra(vec!["+", "*", "-"], vec![2, 3, 4, 5]), 9);
        assert_eq!(do_algebra(vec!["//", "*"], vec![7, 3, 4]), 8);
    }


}
