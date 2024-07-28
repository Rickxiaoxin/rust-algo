// 0-1背包：暴力搜索
fn knapsack_dfs(wgt: &[i32], val: &[i32], i: usize, c: usize) -> i32 {
    // 若已选完物品或背包无剩余容量，返回价值0
    if i == 0 || c == 0 {
        return 0;
    }
    // 若超过背包容量，选择不放入背包
    if wgt[i - 1] > c as i32 {
        return knapsack_dfs(wgt, val, i - 1, c);
    }
    // 计算不放入和放入物品i的最大价值
    let no = knapsack_dfs(wgt, val, i - 1, c);
    let yes = val[i - 1] + knapsack_dfs(wgt, val, i - 1, c - wgt[i - 1] as usize);
    std::cmp::max(no, yes)
}

// 0-1背包：记忆搜索
fn knapsack_dfs_mem(wgt: &[i32], val: &[i32], mem: &mut Vec<Vec<i32>>, i: usize, c: usize) -> i32 {
    // 若已选完所有物品或背包无剩余容量，返回价值0
    if i == 0 || c == 0 {
        return 0;
    }
    // 若已有记录，则直接返回
    if mem[i][c] != -1 {
        return mem[i][c];
    }
    // 若超过背包容量，选择不放入背包
    if wgt[i - 1] > c as i32 {
        return knapsack_dfs_mem(wgt, val, mem, i - 1, c);
    }
    // 计算不放入和放入物品i的最大价值
    let no = knapsack_dfs_mem(wgt, val, mem, i - 1, c);
    let yes = val[i - 1] + knapsack_dfs_mem(wgt, val, mem, i - 1, c - wgt[i - 1] as usize);
    mem[i][c] = std::cmp::max(no, yes);
    mem[i][c]
}

// 0-1背包：动态规划
fn knapsack_dp(wgt: &[i32], val: &[i32], cap: usize) -> i32 {
    let n = wgt.len();
    // 初始化dp表
    let mut dp = vec![vec![0; cap + 1]; n + 1];
    // 状态转移
    for i in 1..=n {
        for c in 1..=cap {
            if wgt[i - 1] > c as i32 {
                // 若超过背包容量，选择不放入背包
                dp[i][c] = dp[i - 1][c];
            } else {
                // 计算不放入和放入物品i的最大价值
                dp[i][c] = std::cmp::max(
                    dp[i - 1][c],
                    dp[i - 1][c - wgt[i - 1] as usize] + val[i - 1],
                );
            }
        }
    }
    dp[n][cap]
}

// 0-1背包：空间优化后的动态规划
fn knapsack_dp_comp(wgt: &[i32], val: &[i32], cap: usize) -> i32 {
    let n = wgt.len();
    // 初始化dp表
    let mut dp = vec![0; cap + 1];
    // 状态转移
    for i in 1..=n {
        for c in (1..=cap).rev() {
            if wgt[i - 1] <= c as i32 {
                // 计算不放入和放入物品i的最大价值
                dp[c] = std::cmp::max(dp[c], dp[c - wgt[i - 1] as usize] + val[i - 1]);
            }
        }
    }
    dp[cap]
}

fn main() {
    let wgt = [10, 20, 30, 40, 50];
    let val = [50, 120, 150, 210, 240];
    let cap: usize = 50;
    let n = wgt.len();

    // 暴力搜索
    let res = knapsack_dfs(&wgt, &val, n, cap);
    println!("不超过背包容量的最大物品价值为 {res}");

    // 记忆搜索
    let mut mem = vec![vec![0; cap + 1]; n + 1];
    for row in mem.iter_mut() {
        row.fill(-1);
    }
    let res = knapsack_dfs_mem(&wgt, &val, &mut mem, n, cap);
    println!("不超过背包容量的最大物品价值为 {res}");

    // 动态规划
    let res = knapsack_dp(&wgt, &val, cap);
    println!("不超过背包容量的最大物品价值为 {res}");

    // 空间优化后的动态规划
    let res = knapsack_dp_comp(&wgt, &val, cap);
    println!("不超过背包容量的最大物品价值为 {res}");
}
