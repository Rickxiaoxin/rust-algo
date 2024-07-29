// 编辑距离：暴力搜索
fn edit_distance_dfs(s: &str, t: &str, i: usize, j: usize) -> i32 {
    // 若s和t都为空，则返回0
    if i == 0 && j == 0 {
        return 0;
    }
    // 若s为空，则返回t的长度
    if i == 0 {
        return j as i32;
    }
    // 若t为空，则返回s的长度
    if j == 0 {
        return i as i32;
    }
    // 若两字符相等，则直接跳过此两字符
    if s.chars().nth(i - 1) == t.chars().nth(j - 1) {
        return edit_distance_dfs(s, t, i - 1, j - 1);
    }
    // 最少编辑步数=插入、删除、替换这三种的最少编辑步数+1
    let insert = edit_distance_dfs(s, t, i, j - 1);
    let delete = edit_distance_dfs(s, t, i - 1, j);
    let replace = edit_distance_dfs(s, t, i - 1, j - 1);
    // 返回最少编辑步数
    std::cmp::min(std::cmp::min(insert, delete), replace) + 1
}

// 编辑距离：记忆化搜索
fn edit_distance_dfs_mem(s: &str, t: &str, mem: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    // 若s和t都为空，则返回0
    if i == 0 && j == 0 {
        return 0;
    }
    // 若s为空，则返回t的长度
    if i == 0 {
        return j as i32;
    }
    // 若t为空，则返回s的长度
    if j == 0 {
        return i as i32;
    }
    // 若有记录，则直接返回
    if mem[i][j] != -1 {
        return mem[i][j];
    }
    // 若两字符相等，则直接跳过此两字符
    if s.chars().nth(i - 1) == t.chars().nth(j - 1) {
        return edit_distance_dfs_mem(s, t, mem, i - 1, j - 1);
    }
    // 最少编辑步数=插入、删除、替换这三种的最少编辑步数+1
    let insert = edit_distance_dfs_mem(s, t, mem, i, j - 1);
    let delete = edit_distance_dfs_mem(s, t, mem, i - 1, j);
    let replace = edit_distance_dfs_mem(s, t, mem, i - 1, j - 1);
    // 记录返回最少编辑步数
    mem[i][j] = std::cmp::min(std::cmp::min(insert, delete), replace) + 1;
    mem[i][j]
}

// 编辑距离：动态规划
fn edit_distance_dp(s: &str, t: &str) -> i32 {
    let (m, n) = (s.len(), t.len());
    // 初始化dp表
    let mut dp = vec![vec![0; n + 1]; m + 1];
    // 初始化边界条件
    // 如果s为空，则返回t的长度
    for i in 0..=n {
        dp[0][i] = i as i32;
    }
    // 如果t为空，则返回s的长度
    for j in 0..=m {
        dp[j][0] = j as i32;
    }
    // 状态转移
    for i in 1..=m {
        for j in 1..=n {
            if s.chars().nth(i - 1) == t.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] =
                    std::cmp::min(std::cmp::min(dp[i - 1][j], dp[i][j - 1]), dp[i - 1][j - 1]) + 1;
            }
        }
    }
    dp[m][n]
}

// 编辑距离：空间优化后的动态规划
fn edit_distance_dp_comp(s: &str, t: &str) -> i32 {
    let (m, n) = (s.len(), t.len());
    // 初始化dp表
    let mut dp = vec![0; n + 1];
    for i in 0..=n {
        dp[i] = i as i32;
    }
    // 状态转移
    for i in 1..=m {
        // 状态转移：首列
        let mut leftup = dp[0]; // 暂存dp[i-1][j-1]
        dp[0] = i as i32;
        // 状态转移：其余列
        for j in 1..=n {
            let temp = dp[j];
            if s.chars().nth(i - 1) == t.chars().nth(j - 1) {
                dp[j] = leftup;
            } else {
                dp[j] = std::cmp::min(std::cmp::min(dp[j], dp[j - 1]), leftup) + 1;
            }
            leftup = temp;
        }
    }
    dp[n]
}

fn main() {
    let s = "bag";
    let t = "pack";
    let (n, m) = (s.len(), t.len());

    // 暴力搜索
    let res = edit_distance_dfs(s, t, n, m);
    println!("将 {s} 更改为 {t} 最少需要编辑 {res} 步");

    // 记忆搜索
    let mut mem = vec![vec![0; m + 1]; n + 1];
    for row in mem.iter_mut() {
        row.fill(-1);
    }
    let res = edit_distance_dfs_mem(s, t, &mut mem, n, m);
    println!("将 {s} 更改为 {t} 最少需要编辑 {res} 步");

    // 动态规划
    let res = edit_distance_dp(s, t);
    println!("将 {s} 更改为 {t} 最少需要编辑 {res} 步");

    // 空间优化后的动态规划
    let res = edit_distance_dp_comp(s, t);
    println!("将 {s} 更改为 {t} 最少需要编辑 {res} 步");
}
