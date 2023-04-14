#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

mod solution1019 {
  use std::collections::HashSet;
  use crate::answer_1019::ListNode;

  pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut now_index = 0;
    let mut stack_:Vec<[i32;2]> = Vec::new();
    let mut return_:Vec<i32> = Vec::new();



    let mut head1 = head;
    while head1 != None{
      // 这里必须有一个变量承载head.unwrap, 不然head.unwrap使用一次之后没有变量承载将失去所有权。
      let head_unwrap = head1.unwrap();
      let val = head_unwrap.val;
      return_.push(0);
      let mut stack_len = stack_.len();
      while stack_len > 0 && val > stack_[stack_len - 1][0]{
        if let Some(node) = stack_.pop(){
          return_[node[1] as usize] = val;
        }
        stack_len = stack_.len();
      }
      stack_.push([val,now_index]);
      now_index += 1;
      head1 = head_unwrap.next;
    }
    return_
  }
}