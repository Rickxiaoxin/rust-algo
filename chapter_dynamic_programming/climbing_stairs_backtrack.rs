// 回溯
fn backtarck(choices: &[i32], state: i32, n: i32, res: &mut [i32]) {
    // 当爬到第n阶时，方案数量加1
    if state == n {
        res[0] = res[0] + 1;
    }
    // 遍历所有选择
    for &choice in choices {
        // 剪枝：不允许超过第n阶
        if state + choice > n {
            continue;
        }
        // 尝试：做出选择，更新状态
        backtarck(choices, state + choice, n, res);
        // 回退
    }
}

// 爬楼梯：回溯
fn climbing_stairs_backtrack(n: usize) -> i32 {
    let choices = vec![1, 2]; // 可选择向上爬1或2阶
    let state = 0; // 从第0阶开始
    let mut res = Vec::new();
    res.push(0); // 使用res[0]记录方案数量
    backtarck(&choices, state, n as i32, &mut res);
    res[0]
}

fn main() {
    let n: usize = 9;

    let res = climbing_stairs_backtrack(n);
    println!("爬 {n} 阶楼梯共有 {res} 种方案");
}
