use std::cmp::max;

// 纯爆破
// 这两天看Rust圣经至少对泛形加深了不少理解
pub fn sum<T:std::ops::Add<Output = T> + std::ops::Sub<Output=T> + Copy> (a:&[T]) -> T{
    let mut p:T = a[0];
    for i in a{
        p = p + *i;
    }
    p - a[0]
}


pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let mut index:usize = 0;
        let first_len = first_len as usize;
        let second_len = second_len as usize;
        let mut max_:i32 = 0;
        let all_len = nums.len();
        while (index < all_len - first_len - second_len + 1){
            let first_ = sum(&nums[index..index + first_len]);
            let mut index2 = index + first_len;
            while (index2 < all_len - second_len + 1){
                let sum_ =  first_ + sum(&nums[index2..index2 + second_len]);
                if max_ < sum_{max_ = sum_;}
                index2 += 1;
            }
            index += 1;
        }
        max_
}

pub fn answer1031(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
    let a = max_sum_two_no_overlap(nums.clone(), first_len.clone(), second_len.clone());
    let b = max_sum_two_no_overlap(nums.clone(), second_len, first_len);
    return max(a,b);
}
#[test]
fn test(){
    assert_eq!(answer1031(vec![4,5,14,16,16,20,7,13,8,15],3, 5), 109)
}
