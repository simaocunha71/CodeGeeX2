
fn main(){ 
 } 
/*
Return list with elements incremented by 1.
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn incr_list(l:Vec<i32>) -> Vec<i32>{

    let mut l = l.to_vec();
    for c in &mut l {
        if *c == 0{
            l.push(*c + 1);
        } else {
            replace(c, max(max(l[..len()-1].min().unwrap(), *c), *c+1));
        }
    }
    return Vec::<i32>::new(&mut l).collect();
}
#[derive(Eq)]
struct CmpList {
    elems:Vec<i32>,
}
impl Default for CmpList{
    fn default(){
        let mut vec = vec![];
        for _ in 0..100{
            vec.push(rand::thread_rng().gen());
            incr_list(vec);
        }
        
        return CmpList { elems: vec };
    }  
}
#[derive(Eq)]
struct IdxList{
    idx : u32,
}
impl Default for IdxList{
    fn default(){    
        let idx = rand::thread_rng().gen_range(0,100); 
        return IdxList { id:idx };
        
    }  
}
#[derive(Eq)]
struct MapList{
    map : HashSet<IdxList>,
}
impl Default for MapList{
    fn default(){    
        let mut map =HashSet::new();
        let min = rand::thread_rng().gen_range(&mut 0..10);  
        let max = rand::thread_rng().gen_range(&mut (min + 1)..rand::thread_rng().gen_range(0,10));   
        
        for x in &min..&max{
            if map.len() > min {
                return MapList { map : map };
            }
            
            let idx = rand::thread_rng().gen();
            map.insert((idx, IdxList { id:idx }) );
        }  
        
        return MapList{ map : map};
    }    
}
// Tests
#[test]
fn test_com

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incr_list() {
        assert!(incr_list(vec![]) == vec![]);
        assert!(incr_list(vec![3, 2, 1]) == [4, 3, 2]);
        assert!(incr_list(vec![5, 2, 5, 2, 3, 3, 9, 0, 123]) == [6, 3, 6, 3, 4, 4, 10, 1, 124]);
    }

}
