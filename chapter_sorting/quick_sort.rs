// 快速排序
struct QuickSort;

impl QuickSort {
    // 哨兵划分
    fn partition(nums: &mut [i32], left: usize, right: usize) -> usize {
        // 以nums[left]为基准数
        let (mut i, mut j) = (left, right);
        while i < j {
            while i < j && nums[j] >= nums[left] {
                j -= 1; // 从右往左找首个小于基准数的元素
            }
            while i < j && nums[j] <= nums[left] {
                i += 1; // 从左往右找首个大于基准数的元素
            }
            nums.swap(i, j); //交换元素
        }
        nums.swap(i, left); // 将基准线交换至两子树组的分界线
        i // 返回基准数的索引
    }

    // 快速排序
    pub fn quick_sort(left: i32, right: i32, nums: &mut [i32]) {
        // 子树组长度为1时终止递归
        if left >= right {
            return;
        }
        // 哨兵划分
        let pivot = Self::partition(nums, left as usize, right as usize) as i32;
        // 递归左子数组，右子数组
        Self::quick_sort(left, pivot - 1, nums);
        Self::quick_sort(pivot + 1, right, nums);
    }
}

fn main() {}
