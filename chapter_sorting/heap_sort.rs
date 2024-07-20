include!("../include/include.rs");

// 堆的长度为n，从节点i开始，从顶至底堆化
fn sift_down(nums: &mut [i32], n: usize, mut i: usize) {
    loop {
        // 判断节点 i,l,r中值最大的节点，即为ma
        let l = 2 * i + 1;
        let r = 2 * i + 2;
        let mut ma = i;
        if l < n && nums[l] > nums[ma] {
            ma = l;
        }
        if r < n && nums[r] > nums[ma] {
            ma = r;
        }
        // 若节点i最大或索引l,r越界，则无须继续堆化，跳出
        if ma == i {
            break;
        }
        // 交换两节点
        nums.swap(i, ma);
        // 循环向下堆化
        i = ma;
    }
}

// 堆排序
fn heap_sort(nums: &mut [i32]) {
    // 建堆操作：堆化除叶节点外的其它所有节点
    for i in (0..nums.len() / 2).rev() {
        sift_down(nums, nums.len(), i);
    }
    // 从堆中提取最大元素，循环n-1轮
    for i in (1..nums.len()).rev() {
        // 交换根节点与最右叶节点（交换首元素与尾元素）
        nums.swap(0, i);
        // 以根节点为起点，从顶至底进行堆化
        sift_down(nums, i, 0);
    }
}

fn main() {
    let mut nums = [4, 1, 3, 1, 5, 2];
    heap_sort(&mut nums);
    print!("堆排序完成后 nums = ");
    print_util::print_array(&nums);
}
