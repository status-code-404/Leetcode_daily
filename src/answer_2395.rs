use crate::answer_2395::answer_2395::answer;

pub mod answer_2395{
    use std::collections::HashMap;

    pub fn answer(nums: Vec<i32>) -> bool{
        if nums.len() == 2{
            return false
        }

        let mut hash = HashMap::new();
        let mut index = 0;
        while index < nums.len() - 1{
            let sum = nums[index] + nums[index+1];
            match hash.get(&sum){
                Some(_) => {return true},
                _ => {
                    hash.insert(sum, true);
                }
            }
            index += 1;
        }
        return false
    }
}

#[test]
fn test(){
    assert_eq!(answer(vec![1,2,3,4,5]), false, "Wrong")
}
