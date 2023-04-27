
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static!{
    static ref WordLen:Mutex<RefCell<HashMap<usize, Vec<String>>>> = Mutex::new(RefCell::new(HashMap::new()));
    static ref WordLink: Mutex<RefCell<HashMap<String, i32>>> = Mutex::new(RefCell::new(HashMap::new()));
}

pub fn judge_two_words(word1:String, word2:String) -> bool{
    let mut index_1:usize = 0;
    let mut index_2:usize = 0;
    let w1_len = word1.len();
    let w2_len = word2.len();
    let w1 = word1.as_bytes();
    let w2 = word2.as_bytes();
    let mut diff = 0;

    while index_1 < word1.len(){
        if w1[index_1]!= w2[index_2] {
            if diff >= 1{
                return false
            }
            if w1[index_1] !=  w2[index_2 + 1] {
                return false
            }
            diff = 1;
            index_2 += 2;
            index_1 += 1;
        }else{
            index_2 += 1;
            index_1 += 1;
        }
    }
    diff <= 1 && w1_len == w2_len - 1
}

pub fn dfs(word:String, round:i32) -> i32{
    let mut word_link_copy = WordLink.lock().unwrap();
    if let Some(round_) = word_link_copy.get_mut().get(&word){
        return round_ + round - 1;
    }
    drop(word_link_copy);
    let mut word_len_copy = WordLen.lock().unwrap();
    if let None = word_len_copy.get_mut().get(&(word.len() + 1)){
        return round;
    }
    let mut last_round = 0;
    let word_list = word_len_copy.get_mut().get(&(word.len() + 1)).unwrap().clone();
    drop(word_len_copy);
    for word2 in word_list{
        if judge_two_words(word.clone(), word2.clone()){
            let r = dfs(word2.clone(), 1);
            if last_round < r{
                last_round = r;
            }
        }
    }
    let mut word_link_copy = WordLink.lock().unwrap();
    word_link_copy.get_mut().insert(word.clone(), last_round + 1);
    last_round + round
}

pub fn answer1048(words: Vec<String>) -> i32 {
    let mut w2 = WordLink.lock().unwrap();
    w2.get_mut().clear();
    drop(w2);

    let mut w1 =WordLen.lock().unwrap();
    w1.get_mut().clear();
    for w in words{
        match w1.get_mut().get(&w.len()){
            Some(word) => {
                let mut word = word.clone();
                word.push(w.clone());
                w1.get_mut().insert(w.len(), word);
            },
            None =>{
                let word = vec![w.clone()];
                w1.get_mut().insert(w.len(), word);
            }
        }
    }


    let mut len_sort:Vec<usize> = Vec::new();
    for len in w1.get_mut().keys(){
        len_sort.push(*len);
    }
    len_sort.sort_by(|a, b| b.cmp(a));
    let len_sort = &len_sort[1..];
    drop(w1);

    let mut result_ = 1;
    for len in len_sort{
        let mut words = Vec::new();
        let mut w1 = WordLen.lock().unwrap();
        if let Some(w) = w1.get_mut().get(len){
            words = w.clone();
        }
        drop(w1);
        for w in words{
            let r = dfs(w.clone(), 1);
            if r > result_{
                result_ = r;
            }
        }
    }
    result_

}


pub fn translate(a:Vec<&str>) -> Vec<String>{
    let mut b = Vec::new();
    for s in a{
        b.push(String::from(s));
    }
    b
}

#[test]
pub fn test_do(){
    let c = translate(vec!["a","b","ba","bca","bda","bdca"]);
    println!("{:?}", c);
    assert_eq!(4, answer1048(c));
}