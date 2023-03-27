mod answer_1574;
mod answer_1626;
mod answer_2395;
mod answer_1638;

use std::cell::RefCell;
use std::collections::HashMap;
use lazy_static::lazy_static;
use answer_1574::answer_1574::solute;
use answer_1626::answer_1626::dfs;
use crate::answer_1626::answer_1626::answer;


fn main(){
    println!("Hello world");
    let mut a = vec![1,2,3,4,5];
    a.remove(0);
    println!("{:?}", a);
    a.remove(0);
    println!("{:?}", a);
    println!("{}", dfs(vec![[1,5], [1,5], [2,4], [2,6]], -1));

    println!("{}", answer(vec![4,5,6,5], vec![2,1,2,1]));
    // let p = solute(vec![1,2,3,10,0,7,8,9]);
    // println!("{}", p)
}
