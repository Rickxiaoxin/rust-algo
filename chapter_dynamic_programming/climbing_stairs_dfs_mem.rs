// 记忆化搜索
fn dfs(i: usize, mem: &mut [i32]) -> i32 {
    // 已知dp[1]和dp[2]
    if i == 1 || i == 2 {
        return i as i32;
    }
    // 若存在记录dp[i],则直接返回
    if mem[i] != -1 {
        return mem[i];
    }
    let count = dfs(i - 1, mem) + dfs(i - 2, mem);
    // 记录dp[i]
    mem[i] = count;
    count
}

// 爬楼梯：记忆化搜索
fn climbing_stairs_dfs_mem(n: usize) -> i32 {
    let mut mem = vec![-1; n + 1];
    dfs(n, &mut mem)
}

fn main() {
    let n: usize = 9;

    let res = climbing_stairs_dfs_mem(n);
    println!("爬 {n} 阶楼梯共有 {res} 种方案");
}
