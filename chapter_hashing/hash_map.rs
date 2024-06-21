include!("../include/include.rs");

use std::collections::HashMap;

fn main() {
    // 初始化哈希表
    let mut map = HashMap::new();

    // 添加操作
    // 在哈希表中添加键值对
    map.insert(12836, "小哈");
    map.insert(15937, "小啰");
    map.insert(16750, "小算");
    map.insert(13276, "小法");
    map.insert(10583, "小鸭");
    println!("添加完成后，哈希表为 map = {:?}", map);

    // 查询操作
    // 向哈希表中输入key,得到value
    let name = map.get(&15937).copied().unwrap();
    println!("输入学号 15937，查询姓名 {}", name);

    // 删除操作
    // 从哈希表中删除键值对
    _ = map.remove(&10583);
    println!("删除10583后，哈希表为 map = {:?}", map);

    //遍历哈希表
    println!("单独遍历key");
    for key in map.keys() {
        println!("{key}");
    }
    println!("单独遍历value");
    for value in map.values() {
        println!("{value}");
    }
}
