mod binary_search_insertion;

use binary_search_insertion::binary_search_insertion;

// 二分查找最左一个target
fn binary_search_left_edge(nums: &[i32], target: i32) -> i32 {
    // 等价于查找target的插入点
    let i = binary_search_insertion(nums, target);
    // 未找到target，返回-1
    if i == nums.len() as i32 || nums[i as usize] != target {
        return -1;
    }
    // 找到target,返回索引i
    i
}

// 二分查找最右一个target
fn binary_search_right_edge(nums: &[i32], target: i32) -> i32 {
    // 转化为查找最左一个target+1
    let i = binary_search_insertion(nums, target + 1);
    // j指向最右一个target，i指向首个大于target的元素
    let j = i - 1;
    // 未找到target，返回-1
    if j == -1 || nums[j as usize] != target {
        return -1;
    }
    // 找到target，返回索引j
    j
}

fn main() {
    // 包含重复元素的数组
    let nums = [1, 3, 6, 6, 6, 6, 6, 10, 12, 15];
    println!("\n数组 nums = {:?}", nums);

    // 二分查找左边界和右边界
    for target in [6, 7] {
        let index = binary_search_left_edge(&nums, target);
        println!("最左一个元素{}的索引为{}", target, index);
        let index = binary_search_right_edge(&nums, target);
        println!("最右一个元素{}的索引为{}", target, index);
    }
}
