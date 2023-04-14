mod answer_1574;
mod answer_1626;
mod answer_2395;
mod answer_1638;
mod answer_1092;
mod answer_1637;
mod answer_1019;
mod answer_1023;

use std::cell::RefCell;
use std::collections::HashMap;
use lazy_static::lazy_static;
use answer_1574::answer_1574::solute;
use answer_1626::answer_1626::dfs;
use crate::answer_1626::answer_1626::answer;


fn main(){
    let mut a = 0;
    println!("{}", (a - 1) % 4);
    let mut h = String::from("ZHello world");
    let mut t = &h[1..2];
    println!("{}", t.as_bytes()[0]);
    let m = h.as_bytes();
    println!("{}", m[2]);
    // let p = solute(vec![1,2,3,10,0,7,8,9]);
    // println!("{}", p)
}
