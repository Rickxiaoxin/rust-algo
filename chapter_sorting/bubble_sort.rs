// 冒泡排序
fn bubble_sort(nums: &mut [i32]) {
    // 外循环：未排序区间为[0,i]
    for i in (1..nums.len()).rev() {
        // 内循环：将未排序区间[0,i]中的最大元素交换到该区间的最右端
        for j in 0..i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}

// 冒泡排序（标志优化）
fn bubble_sort_with_flag(nums: &mut [i32]) {
    // 外循环
    for i in (1..nums.len()).rev() {
        let mut flag = false; // 初始化标志位
        for j in 0..i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                flag = true;
            }
        }
        if !flag {
            break; // 未交换任何元素，直接退出
        }
    }
}

fn main() {
    let mut nums = [4, 1, 3, 1, 5, 2];
    let mut nums1 = nums.clone();
    bubble_sort(&mut nums);
    println!("冒泡排序完成后的nums={:?}", nums);
    bubble_sort_with_flag(&mut nums1);
    println!("冒泡排序完成后的nums1={:?}", nums1);
}
