
fn main(){ 
 } 
/*

    Create a function that takes integers, floats, or strings representing
    real numbers, and returns the larger variable in its given variable type.
    Return None if the values are equal.
    Note: If a real number is represented as a string, the floating point might be . or ,
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn compare_one<'a>(a:&'a dyn Any, b:&'a dyn Any) -> RtnType<String,f64,i32>{

    let id = TypeId::of::<dyn Any>();
    if typeid:: dynamics::dynamic::is_same(Some(&id), Some(b)) {
        compare_one_generic(a.downcast(), b);
    } else if typeid::dynamics::is_any(Some(&id)) {
        compare_none();
    } else {
        let t = typeid::dyname::get::<dyn Any>();
        let id2 = TypeId::of(t);
        match typeid::dynamics::is_any(Some(&id2)) {
            true => {
                compare_none();
            }
            false =>{
                compare_one_generic(a.downcast(), b)
            }
        }
    }
}
fn compare_one_generic<'t,Id:TypeId + 'static>(a:&'t dyn Any, b: &'t dyn Any){
    let _ = replace(&*a, 0);
    let _ = replace(&*b, 0);
    fn _compare2 (x1:&'a i32 , x2 :&'a i32)->cmp::Result<cmp2::Ordering>{
        if cmp::Ordering::Equal == cmp2::ordering(x1.cmp(&*x2)) { return Ok((cmp::Ordering::Equal)); } else{ 
            let _ = replace(&**&x1, 0);
            let _ = replace(&**&x2, 0);
            if x1 > x2 {return Ok(cmp2::ordering::Greater);}else{if cmp2::ordering::Equal == cmp2::ordering(*x1.cmp(&*x2)) {} else return Ok((cmp::Ordering::Less));}}
    }fn _compare3 (a:&'a i32 , b :&'a i32)->cmp::Result<cmp::Ordering>{  
        if cmp::Ordering::Equal == cmp::ordering(b.cmp(&*a)) { return Ok((cmp::Ordering::Equal)); } else{let _ = replace(&**&b, 0); let _ = replace(&**&a, 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_one() {
        assert_eq!(compare_one(&1, &2), RtnType::Int(2));
        assert_eq!(compare_one(&1, &2.5), RtnType::Float(2.5));
        assert_eq!(compare_one(&2, &3), RtnType::Int(3));
        assert_eq!(compare_one(&5, &6), RtnType::Int(6));
        assert_eq!(compare_one(&1, &"2.3"), RtnType::String("2.3".to_string()));
        assert_eq!(compare_one(&"5.1", &"6"), RtnType::String("6".to_string()));
        assert_eq!(compare_one(&"1", &"2"), RtnType::String("2".to_string()));
        assert_eq!(compare_one(&"1", &1), RtnType::String("None".to_string()));
    }

}
