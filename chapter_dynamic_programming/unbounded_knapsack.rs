// 完全背包：动态规划
fn unbounded_knapsack_dp(wgt: &[i32], val: &[i32], cap: usize) -> i32 {
    let n = wgt.len();
    // 初始化dp表
    let mut dp = vec![vec![0; cap + 1]; n + 1];
    // 状态转移
    for i in 1..=n {
        for c in 1..=cap {
            // 若超过背包容量，则不选物品i
            if wgt[i - 1] > c as i32 {
                dp[i][c] = dp[i - 1][c];
            } else {
                // 不选和选物品i两种方案的较大值
                dp[i][c] = std::cmp::max(dp[i - 1][c], dp[i][c - wgt[i - 1] as usize] + val[i - 1]);
            }
        }
    }
    dp[n][cap]
}

// 完全背包：空间优化后的动态规划
fn unbounded_knapsack_dp_comp(wgt: &[i32], val: &[i32], cap: usize) -> i32 {
    let n = wgt.len();
    let mut dp = vec![0; cap + 1];
    for i in 1..=n {
        for c in 1..=cap {
            if wgt[i - 1] > c as i32 {
                dp[c] = dp[c];
            } else {
                dp[c] = std::cmp::max(dp[c], dp[c - wgt[i - 1] as usize] + val[i - 1]);
            }
        }
    }
    dp[cap]
}

fn main() {
    let wgt = [1, 2, 3];
    let val = [5, 11, 15];
    let cap: usize = 4;

    // 动态规划
    let res = unbounded_knapsack_dp(&wgt, &val, cap);
    println!("不超过背包容量的最大物品价值为 {res}");

    // 空间优化后的动态规划
    let res = unbounded_knapsack_dp_comp(&wgt, &val, cap);
    println!("不超过背包容量的最大物品价值为 {res}");
}
