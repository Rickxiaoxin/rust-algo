// 最大切分乘积：贪心
fn max_product_cutting(n: i32) -> i32 {
    // 当n<=3时，必须切分出一个1
    if n <= 3 {
        return 1 * (n - 1);
    }
    // 贪心地切分出3,a为3的个数，b为余数
    let a = n / 3;
    let b = n % 3;
    if b == 1 {
        // 当余数为1时，将1+3切分出两个2
        3i32.pow(a as u32 - 1) * 2 * 2
    } else if b == 2 {
        3i32.pow(a as u32) * 2
    } else {
        3i32.pow(a as u32)
    }
}

fn main() {
    let n = 58;

    // 贪心算法
    let res = max_product_cutting(n);
    println!("最大切分乘积为 {}", res);
}
