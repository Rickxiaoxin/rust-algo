include!("../include/include.rs");

// 计数排序
// 简单实现，无法用于排序对象
fn counting_sort_naive(nums: &mut [i32]) {
    // 1.统计数组最大元素 m
    let m = *nums.iter().max().unwrap();
    // 2.统计各个数字出现此书
    // counter[num]代表num出现的次数
    let mut counter = vec![0; m as usize + 1];
    for &num in nums.iter() {
        counter[num as usize] += 1;
    }
    // 3.遍历counter，将各元素填入原数组nums
    let mut i = 0;
    for num in 0..m + 1 {
        for _ in 0..counter[num as usize] {
            nums[i] = num;
            i += 1;
        }
    }
}

// 完整实现，可排序对象，且稳定排序
fn counting_sort(nums: &mut [i32]) {
    // 1.统计数组最大元素 m
    let m = *nums.iter().max().unwrap() as usize;
    // 2.统计各个数字出现的次数
    let mut counter = vec![0; m + 1];
    for &num in nums.iter() {
        counter[num as usize] += 1;
    }
    // 3.求counter的前缀和，将出现次数转换为尾索引
    for i in 0..m {
        counter[i + 1] += counter[i];
    }
    // 4.倒序遍历nums，将各元素填入结果数组
    let n = nums.len();
    let mut res = vec![0; n];
    for i in (0..n).rev() {
        let num = nums[i];
        res[counter[num as usize] - 1] = num; // 将num放置到对应索引处
        counter[num as usize] -= 1; // 减少计数
    }
    // 使用结果数组res覆盖元数组nums
    nums.copy_from_slice(&res);
}

fn main() {
    let mut nums = [1, 0, 1, 2, 0, 4, 0, 2, 2, 4];
    counting_sort_naive(&mut nums);
    print!("计数排序（无法排序对象）完成后 nums = {:?}", nums);

    let mut nums1 = [1, 0, 1, 2, 0, 4, 0, 2, 2, 4];
    counting_sort(&mut nums1);
    print!("\n计数排序完成后 nums1 = {:?}", nums1);
}
