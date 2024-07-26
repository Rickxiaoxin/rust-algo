// 搜索
fn dfs(i: usize) -> i32 {
    // 已知dp[1]和dp[2],返回
    if i == 1 || i == 2 {
        return i as i32;
    }
    // dp[i]=dp[i-1]+dp[i-2]
    let count = dfs(i - 1) + dfs(i - 2);
    count
}

// 爬楼梯：搜索
fn climbing_stairs_dfs(n: usize) -> i32 {
    dfs(n)
}

fn main() {
    let n = 9;
    let res = climbing_stairs_dfs(n);
    println!("爬 {n} 阶楼梯共有 {res} 种方案");
}
