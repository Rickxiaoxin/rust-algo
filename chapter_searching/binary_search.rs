// 二分查找（双闭区间）
fn binary_search(nums: &[i32], target: i32) -> i32 {
    // 初始化双闭区间[0,n-1],即i,j分别指向数组首元素，尾元素
    let mut i = 0;
    let mut j = nums.len() as i32 - 1;
    // 循环，当搜索区间为空是跳出（即i>j）
    while i <= j {
        let m = i + (j - i) / 2; //计算中点索引
        if nums[m as usize] < target {
            i = m + 1;
        } else if nums[m as usize] > target {
            j = m - 1;
        } else {
            return m;
        }
    }
    // 未找到目标元素，返回-1
    return -1;
}

// 二分查找（左闭右开区间）
fn binary_search_lcro(nums: &[i32], target: i32) -> i32 {
    let mut i = 0;
    let mut j = nums.len() as i32;
    while i <= j {
        let m = i + (j - i) / 2;
        if target < nums[m as usize] {
            j = m;
        } else if target > nums[m as usize] {
            i = m + 1;
        } else {
            return m;
        }
    }
    return -1;
}

fn main() {
    let target = 6;
    let nums = [1, 3, 6, 8, 12, 15, 23, 26, 31, 35];

    // 二分查找（双闭区间）
    let mut index = binary_search(&nums, target);
    println!("目标元素 6 的索引 = {index}");

    // 二分查找（左闭右开区间）
    index = binary_search_lcro(&nums, target);
    println!("目标元素 6 的索引 = {index}");
}
