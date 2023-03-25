use crate::solute;

pub mod answer_1574{
    use std::cmp::min;

    pub fn solute(arr: Vec<i32>) -> i32{
        let mut left_index = 0;
        let mut right_index = arr.len() - 1;
        let mut break1 = false;
        let mut break2 = false;
        let mut iter = 1;
        while iter < arr.len(){
            iter += 1;
            if arr[left_index] <= arr[left_index + 1]{
                left_index +=1;
            }else{
                break1 = true
            }

            if arr[right_index] >= arr[right_index - 1]{
                right_index -= 1;
            }else{
                break2 = true;
            }

            if break1 && break2{
                break;
            }
        }

        let mut min_ = min(right_index, arr.len() - left_index - 1);

        let mut a = 0;
        while a <= left_index{
            while right_index < arr.len() && arr[a] > arr[right_index]{
                right_index += 1;
            }
            min_ = min(min_, right_index - a - 1);
            a += 1;
        }


        return min_ as i32;
    }

}

#[test]
fn test() {
    assert_eq!(solute(vec![1, 2, 3, 10, 0, 7, 8, 9]), 2, "Wrong");
}