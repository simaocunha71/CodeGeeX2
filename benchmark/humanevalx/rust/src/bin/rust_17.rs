
fn main(){ 
 } 
/*
 Input to this function is a string representing musical notes in a special ASCII format.
    Your task is to parse this string and return list of integers corresponding to how many beats does each
    not last.

    Here is a legend:
    'o' - whole note, lasts four beats
    'o|' - half note, lasts two beats
    '.|' - quater note, lasts one beat
    
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn parse_music(music_string:String) -> Vec<i32>{

    let r = Rc::new(RefCell::new(Vec::with_capacity(1 << 9)));
    let mut v : Vec<i32> =Vec::new();
    music_string.chars().for_each(|c| {
        if c == 'o' || c == '0' || c == '|' || c == '.|' ||c == 'x' || c == 'X' || c == '#'  ||c == '@'|| c == '$'{
             let t = TypeId::of::<i32>();
            match (t).clone().into_raw() {
                &i => v.push(i),
                _ => (),
            }
        }else if music_string.chars().all(|c| c == 'o' || c  =='.'){
             let t = TypeId::of::<usize>();
            match (t).clone().into_raw() {
                &i => r.borrow_mut()[((1 << 9) - 2).saturating_sub(r.borrow().len()) as usize].push(i);
                _=> { },
            }
        }else{return;},
    });
    v.sort_by(|a,b| max(a, b));
    Ok(v)
}
#[test]
fn test() {
    parse_music("o24.").unwrap();  // whole note lasts four beats
    let mut r = Rc::new(RefCell::new(Vec::with_capability(1 << 9)));
    parse_music("o36.|").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_parse_music() {
        assert!(parse_music(" ".to_string()) == []);
        assert!(parse_music("o o o o".to_string()) == vec![4, 4, 4, 4]);
        assert!(parse_music(".| .| .| .|".to_string()) == vec![1, 1, 1, 1]);
        assert!(parse_music("o| o| .| .| o o o o".to_string()) == vec![2, 2, 1, 1, 4, 4, 4, 4]);
        assert!(parse_music("o| .| o| .| o o| o o|".to_string()) == vec![2, 1, 2, 1, 4, 2, 4, 2]);
    }

}
