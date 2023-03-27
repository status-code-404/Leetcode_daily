use crate::answer_1638::answer_1638::answer_easy;

pub mod answer_1638{
    use std::collections::HashMap;
    use std::iter::zip;

    pub fn string_diff1(a:&str, b:&str) ->bool{
        if a == b{
            return false
        }
        let mut diff = 0;

        for c in zip(a.chars(), b.chars()){
            if c.0 != c.1 && diff == 1{
                return false
            }else if c.0 != c.1{
                diff = 1;
            }
        }

        return diff == 1;
    }

    pub fn answer_easy(s:String, t:String) -> i32{
        let mut ss;
        let mut tt;
        if t.len() < s.len(){
            ss = t;
            tt = s;
        }else{
            ss = s;
            tt = t;
        }

        let mut left_start_index = 0;
        let mut left_end_index = left_start_index.clone() + 1;
        let mut dict = HashMap::new();
        let mut return_ = 0;
        while left_start_index < ss.len() {
            left_end_index = left_start_index + 1;
            while left_end_index <= ss.len(){
                let mut left_str = &ss[left_start_index..left_end_index];
                match dict.get(left_str){
                    Some(diff) =>{return_ += diff; left_end_index += 1;continue},
                    _ =>{}
                }

                let str_size = left_str.len();
                let mut now_count = 0;
                //
                let mut right_index = 0;
                while right_index < tt.len() - str_size + 1{
                    if string_diff1(left_str.clone(), &tt.clone()[right_index..right_index + str_size]){
                        now_count += 1;
                    }
                    right_index += 1;
                }

                dict.insert(left_str, now_count.clone());
                return_ += now_count;
                left_end_index += 1;
            }
            left_start_index += 1;
        }

        return return_
    }
}

#[test]
pub fn test_1638(){
    let h = answer_easy(String::from("aba"), String::from("baba"));
    println!("{}", h);
}