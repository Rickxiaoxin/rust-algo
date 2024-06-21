use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::rc::Rc;

use crate::list_node::ListNode;

// 打印数组
pub fn print_array<T: Display>(nums: &[T]) {
    print!("[");
    if nums.len() > 0 {
        for (i, num) in nums.iter().enumerate() {
            print!("{}{}", num, if i == nums.len() - 1 { "]" } else { ", " });
        }
    } else {
        print!("]");
    }
}

// 打印链表
pub fn print_linked_list<T: Display>(head: &Rc<RefCell<ListNode<T>>>) {
    print!(
        "{}{}",
        head.borrow().val,
        if head.borrow().next.is_none() {
            "\n"
        } else {
            " -> "
        }
    );
    if let Some(node) = &head.borrow().next {
        return print_linked_list(node);
    }
}

// 打印队列
pub fn print_queue<T: Display>(queue: &VecDeque<T>) {
    print!("[");
    let iter = queue.iter();
    for (i, data) in iter.enumerate() {
        print!("{}{}", data, if i == queue.len() - 1 { "]" } else { ", " });
    }
}
