pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
    let mut return_ = Vec::new();
    for q in queries{
        let q = q.as_bytes();
        let mut index:usize = 0;
        return_.push(true);
        let l = return_.len();
        for p in pattern.as_bytes(){
            while index < q.len() && q[index] != *p {
                if q[index] <= 90{
                    return_[l - 1] = false;
                    break;
                }
                index += 1;
            }

            if !return_[l - 1]{
                break;
            }

            if index == q.len(){
                let l = return_.len();
                return_[l - 1] = false;
                break;
            }

            if q[index] == *p{
                index += 1;
            }
        }


        if !return_[l - 1]{
            continue;
        }

        while index < q.len(){
            if q[index] <= 90{
                return_[l - 1] = false;
                break;
            }
            index += 1;
        }

    }
    return_

}

#[test]
pub fn test_1023(){
    let insert_1 = Vec::from([String::from("FooBar"),String::from("FooBarTest"),
        String::from("FootBall"),String::from("FrameBuffer"),String::from("ForceFeedBack")]);
    let insert_2 = String::from("FoBaT");
    let mut a = Vec::from([false, true,false,false,false]);
    assert_eq!(a, camel_match(insert_1, insert_2))
}

