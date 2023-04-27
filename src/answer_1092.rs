use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::answer_1092::answer_1092::answer;
#[recursion_limit = "2000"]


struct S(String);


impl S {
    pub fn change(&mut self, a:String){
       self.0 = a;
    }

    pub fn get_string(&mut self) -> String{
        return self.0.clone();
    }
}

lazy_static!{
    static ref Hashmap: Mutex<RefCell<HashMap<[i32;2],String>>> = Mutex::new(RefCell::new(HashMap::new()));
    static ref String1:Mutex<RefCell<S>> =  Mutex::new(RefCell::new(S(String::new())));
    static ref String2:Mutex<RefCell<S>> = Mutex::new(RefCell::new(S(String::new())));
}


pub mod answer_1092{
    use std::collections::HashMap;
    use std::ops::Add;
    use crate::answer_1092::{Hashmap, String1, String2};


    pub fn dfs(index_left: i32, index_right: i32) -> String{



        let mut hash = Hashmap.lock().unwrap();

        match hash.get_mut().get(&[index_left.clone(), index_right.clone()]) {
            Some(string) => {return string.clone();},
            _ =>{},
        }
        drop(hash);
        let mut s1 = String1.lock().unwrap();
        let mut s2 = String2.lock().unwrap();

        let mut s1_string = s1.get_mut().get_string();
        let mut s2_string = s2.get_mut().get_string();

        drop(s1);
        drop(s2);

        if index_left as usize == s1_string.len(){
            let mut hash = Hashmap.lock().unwrap();
            let value = &s2_string.clone()[index_right as usize..];
            hash.get_mut().insert([index_left, index_right], String::from(value.clone()));
            drop(hash);
            return String::from(value);
        }

        if index_right as usize == s2_string.len(){
            let mut hash = Hashmap.lock().unwrap();
            let value = &s1_string.clone()[index_left as usize..];
            hash.get_mut().insert([index_left, index_right], String::from(value.clone()));
            drop(hash);
            return String::from(value);
        }

        let s1_str:&str = &s1_string[index_left as usize..(index_left+1) as usize];
        let s2_str:&str = &s2_string[index_right as usize..(index_right+1) as usize];


        if s1_str == s2_str{
            let mut str = dfs(index_left + 1, index_right + 1);
            str = String::from(s1_str).add(str.as_str());
            let mut hash = Hashmap.lock().unwrap();
            hash.get_mut().insert([index_left, index_right], str.clone());
            drop(hash);
            return str;
        } else {
            let ss1 = String::from(s1_str).add(dfs(index_left + 1, index_right).as_str());
            let ss2 = String::from(s2_str).add(dfs(index_left, index_right+ 1).as_str());
            let mut hash = Hashmap.lock().unwrap();
            if ss1.len() < ss2.len(){
                hash.get_mut().insert([index_left, index_right], ss1.clone());
                drop(hash);
                return ss1;
            }else{
                hash.get_mut().insert([index_left, index_right], ss2.clone());
                drop(hash);
                return ss2;
            }

        }
        return String::new();

    }

    pub fn answer(a: String, b: String) -> String{
        let mut hash = Hashmap.lock().unwrap();
        hash.get_mut().clear();
        drop(hash);

        let mut s1 = String1.lock().unwrap();
        s1.get_mut().change(a);
        drop(s1);

        let mut s2 = String2.lock().unwrap();
        s2.get_mut().change(b);
        drop(s2);

        dfs(0,0)
    }
}

#[test]
pub fn test(){
    println!("123");
    println!("{}", answer(String::from("atdznrqfwlfbcqkezrltzyeqvqemikzgghxkzenhtapwrmrovwtpzzsyiwongllqmvptwammerobtgmkpowndejvbuwbporfyroknrjoekdgqqlgzxiisweeegxajqlradgcciavbpgqjzwtdetmtallzyukdztoxysggrqkliixnagwzmassthjecvfzmyonglocmvjnxkcwqqvgrzpsswnigjthtkuawirecfuzrbifgwolpnhcapzxwmfhvpfmqapdxgmddsdlhteugqoyepbztspgojbrmpjmwmhnldunskpvwprzrudbmtwdvgyghgprqcdgqjjbyfsujnnssfqvjhnvcotynidziswpzhkdszbblustoxwtlhkowpatbypvkmajumsxqqunlxxvfezayrolwezfzfyzmmneepwshpemynwzyunsxgjflnqmfghsvwpknqhclhrlmnrljwabwpxomwhuhffpfinhnairblcayygghzqmotwrywqayvvgohmujneqlzurxcpnwdipldofyvfdurbsoxdurlofkqnrjomszjimrxbqzyazakkizojwkuzcacnbdifesoiesmkbyffcxhqgqyhwyubtsrqarqagogrnaxuzyggknksrfdrmnoxrctntngdxxechxrsbyhtlbmzgmcqopyixdomhnmvnsafphpkdgndcscbwyhueytaeodlhlzczmpqqmnilliydwtxtpedbncvsqauopbvygqdtcwehffagxmyoalogetacehnbfxlqhklvxfzmrjqofaesvuzfczeuqegwpcmahhpzodsmpvrvkzxxtsdsxwixiraphjlqawxinlwfspdlscdswtgjpoiixbvmpzilxrnpdvigpccnngxmlzoentslzyjjpkxemyiemoluhqifyonbnizcjrlmuylezdkkztcphlmwhnkdguhelqzjgvjtrzofmtpuhifoqnokonhqtzxmimp"), String::from("xjtuwbmvsdeogmnzorndhmjoqnqjnhmfueifqwleggctttilmfokpgotfykyzdhfafiervrsyuiseumzmymtvsdsowmovagekhevyqhifwevpepgmyhnagjtsciaecswebcuvxoavfgejqrxuvnhvkmolclecqsnsrjmxyokbkesaugbydfsupuqanetgunlqmundxvduqmzidatemaqmzzzfjpgmhyoktbdgpgbmjkhmfjtsxjqbfspedhzrxavhngtnuykpapwluameeqlutkyzyeffmqdsjyklmrxtioawcrvmsthbebdqqrpphncthosljfaeidboyekxezqtzlizqcvvxehrcskstshupglzgmbretpyehtavxegmbtznhpbczdjlzibnouxlxkeiedzoohoxhnhzqqaxdwetyudhyqvdhrggrszqeqkqqnunxqyyagyoptfkolieayokryidtctemtesuhbzczzvhlbbhnufjjocporuzuevofbuevuxhgexmckifntngaohfwqdakyobcooubdvypxjjxeugzdmapyamuwqtnqspsznyszhwqdqjxsmhdlkwkvlkdbjngvdmhvbllqqlcemkqxxdlldcfthjdqkyjrrjqqqpnmmelrwhtyugieuppqqtwychtpjmloxsckhzyitomjzypisxzztdwxhddvtvpleqdwamfnhhkszsfgfcdvakyqmmusdvihobdktesudmgmuaoovskvcapucntotdqxkrovzrtrrfvoczkfexwxujizcfiqflpbuuoyfuoovypstrtrxjuuecpjimbutnvqtiqvesaxrvzyxcwslttrgknbdcvvtkfqfzwudspeposxrfkkeqmdvlpazzjnywxjyaquirqpinaennweuobqvxnomuejansapnsrqivcateqngychblywxtdwntancarldwnloqyywrxrganyehkglbdeyshpodpmdchbcc")));

}