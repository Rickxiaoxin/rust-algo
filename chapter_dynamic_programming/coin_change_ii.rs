// 零钱兑换 2：动态规划
fn coin_change_ii_dp(coins: &[i32], amt: usize) -> i32 {
    let n = coins.len();
    let mut dp = vec![vec![0; amt + 1]; n + 1];
    // 状态转移：首行首列
    for i in 1..=n {
        dp[i][0] = 1;
    }
    dp[0].fill(0);
    // 状态转移：其余行和列
    for i in 1..=n {
        for a in 1..=amt {
            if coins[i - 1] > a as i32 {
                dp[i][a] = dp[i - 1][a];
            } else {
                dp[i][a] = dp[i - 1][a] + dp[i][a - coins[i - 1] as usize]
            }
        }
    }
    dp[n][amt]
}

// 零钱兑换 2：空间优化后的动态规划
fn coin_change_ii_dp_comp(coins: &[i32], amt: usize) -> i32 {
    let n = coins.len();
    let mut dp = vec![0; amt + 1];
    dp[0] = 1;
    for i in 1..=n {
        for a in 1..=amt {
            if coins[i - 1] > a as i32 {
                dp[a] = dp[a];
            } else {
                dp[a] = dp[a] + dp[a - coins[i - 1] as usize];
            }
        }
    }
    dp[amt]
}

fn main() {
    let coins = [1, 2, 5];
    let amt: usize = 5;

    // 动态规划
    let res = coin_change_ii_dp(&coins, amt);
    println!("凑出目标金额的硬币组合数量为 {res}");

    // 空间优化后的动态规划
    let res = coin_change_ii_dp_comp(&coins, amt);
    println!("凑出目标金额的硬币组合数量为 {res}");
}
