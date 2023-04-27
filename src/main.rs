#![allow(unused_imports)]
mod answer_1574;
mod answer_1626;
mod answer_2395;
mod answer_1638;
mod answer_1092;
mod answer_1637;
mod answer_1019;
mod answer_1023;
mod answer_1031;
mod answer_1048;

use std::cell::RefCell;
use std::clone::Clone;
use std::collections::HashMap;
use lazy_static::lazy_static;
use answer_1574::answer_1574::solute;
use answer_1626::answer_1626::dfs;
use crate::answer_1626::answer_1626::answer;


fn sum<T:std::ops::Add<Output = T> + std::ops::Sub<Output=T> + Copy> (a:&[T]) -> T{
    let mut p:T = a[0];
    for i in a{
        p = p + *i;
    }
    p - a[0]
}

fn main(){
    let mut a = 0;
    println!("{}", (a - 1) % 4);
    let mut h = String::from("ZHello world");



    let mut _m = vec![String::from("1234"), String::from("5678")];

    let k = h.as_bytes();
    println!("{}", k[5]);
    let mut t = &h[1..2];
    println!("{}", t.as_bytes()[0]);
    let m = h.as_bytes();
    let q = h.clone();
    println!("{}", m[2]);
    // let p = solute(vec![1,2,3,10,0,7,8,9]);
    // println!("{}", p)

    let mut d = vec![1,2,3,4,5];

    d.sort_by(|a,b| b.cmp(a));
    println!("{}{:?}", sum(&d[1..3]), &d[1..3]);
}
