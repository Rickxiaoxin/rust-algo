// 获取元素num的第k位，其中exp=10^(k-1)
fn digit(num: i32, exp: i32) -> usize {
    return ((num / exp) % 10) as usize;
}

// 计数排序
fn counting_sort_digit(nums: &mut [i32], exp: i32) {
    // 十进制的位范围为0~9,因此需要长度为10的桶数组
    let mut counter = [0; 10];
    let n = nums.len();
    // 1.计数
    for i in 0..n {
        let d = digit(nums[i], exp); // 获取nums[i]第k位，记为d
        counter[d] += 1; // 统计数字d出现的次数
    }
    // 2.前缀和
    for i in 1..10 {
        counter[i] += counter[i - 1];
    }
    // 3.倒序遍历，根据桶内统计结果，将各元素填入res
    let mut res = vec![0; n];
    for i in (0..n).rev() {
        let d = digit(nums[i], exp);
        let j = counter[d] - 1; // 获取d在数组中的索引j
        res[j] = nums[i]; // 将当前元素填入索引j
        counter[d] -= 1; // 将d的数量减1
    }
    // 使用结果覆盖原数组nums
    nums.copy_from_slice(&res);
}

// 基数排序
fn radix_sort(nums: &mut [i32]) {
    // 获取数组的最大元素，用于判断最大位数
    let m = *nums.into_iter().max().unwrap();
    // 按照从低位到高位的顺序遍历
    let mut exp = 1;
    while exp <= m {
        counting_sort_digit(nums, exp);
        exp *= 10;
    }
}

fn main() {
    // 基数排序
    let mut nums = [
        10546151, 35663510, 42865989, 34862445, 81883077, 88906420, 72429244, 30524779, 82060337,
        63832996,
    ];
    radix_sort(&mut nums);
    print!("基数排序完成后 nums = {:?}", nums);
}
