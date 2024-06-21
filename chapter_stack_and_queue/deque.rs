include!("../include/include.rs");

use std::collections::VecDeque;

fn main() {
    // 初始化双向队列
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_back(3);
    deque.push_back(2);
    deque.push_back(5);
    println!("双向队列 deque = {:?}", deque);

    // 访问元素
    let peek_first = deque.front().unwrap();
    println!("队首元素 peekfirst = {}", peek_first);
    let peek_last = deque.back().unwrap();
    println!("队尾元素 peeklast = {}", peek_last);

    // 元素入队
    deque.push_back(4);
    println!("元素 4 队尾入队后 deque = {:?}", deque);
    deque.push_front(1);
    println!("元素 1 队首入队后 deque = {:?}", deque);

    // 元素出队
    let pop_last = deque.pop_back().unwrap();
    println!(
        "队尾出队元素 = {:?},队尾出队后 deque = {:?}",
        pop_last, deque
    );
    let pop_first = deque.pop_front().unwrap();
    println!(
        "队首出队元素 = {:?},队首出队后 deque = {:?}",
        pop_first, deque
    );

    // 获取队列的长度
    let size = deque.len();
    println!("双向队列的长度 size = {}", size);

    // 判断双向队列是否为空
    let is_empty = deque.is_empty();
    println!("双向队列是否为空 = {}", is_empty);
}
