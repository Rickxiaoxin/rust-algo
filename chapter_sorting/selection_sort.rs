fn selection_sort(nums: &mut [i32]) {
    if nums.is_empty() {
        return;
    }
    let n = nums.len();
    for i in 0..n - 1 {
        let mut k = i;
        for j in i + 1..n {
            if nums[j] < nums[k] {
                k = j; // 记录最小元素的索引
            }
        }
        // 将该最小元素与未排序区间的首个元素交换
        nums.swap(i, k);
    }
}

fn main() {
    let mut nums = [4, 1, 3, 1, 5, 2];
    selection_sort(&mut nums);
    println!("排序后的数组 nums = {:?}", nums);
}
