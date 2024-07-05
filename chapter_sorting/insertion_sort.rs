// 插入排序
fn insertion_sort(nums: &mut [i32]) {
    // 外循环
    for i in 1..nums.len() {
        let (base, mut j) = (nums[i], (i - 1) as i32);
        // 内循环
        while j >= 0 && nums[j as usize] > base {
            nums[(j + 1) as usize] = nums[j as usize]; // 将nums[j]向右移动一位
            j -= 1;
        }
        nums[(j + 1) as usize] = base; // 将base赋值到正确位置
    }
}

fn main() {
    let mut nums = [4, 1, 3, 1, 5, 2];
    insertion_sort(&mut nums);
    println!("插入排序后的nums={:?}", nums);
}
