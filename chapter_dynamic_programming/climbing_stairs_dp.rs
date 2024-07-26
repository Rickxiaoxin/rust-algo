use std::vec;

// 爬楼梯：动态规划
fn climbing_stairs_dp(n: usize) -> i32 {
    // 已知dp[1]和dp[2]
    if n == 1 || n == 2 {
        return n as i32;
    }
    // 初始化dp表，用于存储子问题的解
    let mut dp = vec![-1; n + 1];
    // 初始状态：预设最小子问题的解
    dp[1] = 1;
    dp[2] = 2;
    // 状态转移
    for i in 3..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}

/* 爬楼梯：空间优化后的动态规划 */
fn climbing_stairs_dp_comp(n: usize) -> i32 {
    if n == 1 || n == 2 {
        return n as i32;
    }
    let (mut a, mut b) = (1, 2);
    for _ in 3..=n {
        let tmp = b;
        b = a + b;
        a = tmp;
    }
    b
}

fn main() {
    let n: usize = 9;

    let res = climbing_stairs_dp(n);
    println!("爬 {n} 阶楼梯共有 {res} 种方案");

    let res = climbing_stairs_dp_comp(n);
    println!("爬 {n} 阶楼梯共有 {res} 种方案");
}
