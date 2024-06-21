#![allow(unused)]

include!("../include/include.rs");

use list_node::ListNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
// use tree_node::TreeNode;

// 常数阶
fn constant(n: i32) -> i32 {
    _ = n;
    let mut count = 0;
    let size = 100_000;
    for _ in 0..size {
        count += 1;
    }
    count
}

fn main() {}
