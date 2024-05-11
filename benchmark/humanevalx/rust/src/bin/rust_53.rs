
fn main(){ 
 } 
/*
Add two numbers x and y
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn add(x:i32, y:i32) -> i32{

	let (x,y)=(x,y).tuple();//get x and y as tuple
	for (i,(j,$v)) in (0..x).zip((1..y)).map(|(a,b)|(a.cmp(&b)).unwrap_or(max(&a,&b)))
	{
		if i == 0 && j > $v { x = max(x,j); continue; }//change max to min if it is not the first number and j is bigger than v or it was added to the set before
		let (x1,$v1)=(i-1).tuple();//get previous value of x and previous value of the sum and remove it from the hashmap
		if let Some(x)=x.deref() { let $v2=$v; $(
			$v1=replace($v,$v1,[||] for _ in 0..1); if j>=1 {
			$x1=$x.$($v1,j-1); } else {$x1=$j;} $v1=j as i32; $v2=${v:0}.set(i, x.clone());
			let r=[||]; let (x,$v)=r.tuple();
			if let Some($v) = replace(&$v2,[||]); $(let $v1=$v.deref(), let x=$x.$($v2,$v1); {
				if i == 0 && j > $v || x > y { return max(max(x,y),j) + (j - min(min($v.get(i as u32).unwrap() & ((x >> i) | 1)), min(x.$($v,$v1)) & ((y >> j) | 1)));
			} else {$v=replace(&$v,0,[$||]; x=$x; $v.set(j,x.clone());
				let r=[||]; let (r_x,$r_x)=r.tuple(); for $i in 0..j { if i==1 && !r_x.$($v,i) { return max(max(r.$($v,$v1),y).min(x.$(&

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert!(add(0, 1) == 1);
        assert!(add(1, 0) == 1);
        assert!(add(2, 3) == 5);
        assert!(add(5, 7) == 12);
        assert!(add(7, 5) == 12);
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let mut x: i32 = rng.gen();
            x = x % 1000;
            let mut y: i32 = rng.gen();
            y = y % 1000;

            assert!(add(x, y) == x + y);
        }
    }

}
