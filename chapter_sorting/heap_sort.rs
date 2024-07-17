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

fn main() {}
