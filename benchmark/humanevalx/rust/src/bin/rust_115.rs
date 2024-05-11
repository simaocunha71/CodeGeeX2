
fn main(){ 
 } 
/*
    You are given a rectangular grid of wells. Each row represents a single well,
    and each 1 in a row represents a single unit of water.
    Each well has a corresponding bucket that can be used to extract water from it, 
    and all buckets have the same capacity.
    Your task is to use the buckets to empty the wells.
    Output the number of times you need to lower the buckets.
*/
use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

fn max_fill(grid:Vec<Vec<i32>>, capacity:i32) -> i32{

    let mut count = 0;
    for (index1,v1) in grid.iter().enumerate(){
        if v1>capacity-1 {
            v1=capacity-1;
        }
        if v1==capacity {
            count+=1;
        }
    }
    
    return max(count,0);
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    for i in 0..256{
        for j in 0..256{
            if md5(&format!("{}\n",(i,j))).to_string().len()>17 {
                let mut grid = Vec::<Vec<i32>>::new();
                for k in 0..264-2 {
                    let x:i32 = rng.gen_range(1,3);
                    let y:i32 = rng.gen_range(1,3);
                    if (x*y) % 4 == 0 {
                        continue;
                    }
                if x!=y && grid.iter().any(|&&(v,_)==((*v),(*v)),|v.len()>1 && v[0]==x&&v[1]==y){
                    *grid.push(&mut[(x, y)].to_string()) += 1;
                }else{
                   grid.push(Vec::new());}
                }
                let capacity = rng.gen_range(0..256);
                if max(max(capacity,0),0)<9 {
                    println!("{}",capacity+" "+format!("{:b}",(i,j))));
                }else{ continue; }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_fill() {
        assert!(
            max_fill(
                vec![vec![0, 0, 1, 0], vec![0, 1, 0, 0], vec![1, 1, 1, 1]],
                1
            ) == 6
        );
        assert!(
            max_fill(
                vec![
                    vec![0, 0, 1, 1],
                    vec![0, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![0, 1, 1, 1]
                ],
                2
            ) == 5
        );
        assert!(max_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 5) == 0);
        assert!(max_fill(vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1]], 2) == 4);
        assert!(max_fill(vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1]], 9) == 2);
    }

}
