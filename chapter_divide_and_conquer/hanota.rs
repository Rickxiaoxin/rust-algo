#![allow(non_snake_case)]

// 移动一个圆盘
fn move_pan(src: &mut Vec<i32>, tar: &mut Vec<i32>) {
    // 从src顶部拿出一个圆盘
    let pan = src.pop().unwrap();
    // 将圆盘放入tar顶部
    tar.push(pan);
}

// 求解汉诺塔问题f(i)
fn dfs(i: i32, src: &mut Vec<i32>, buf: &mut Vec<i32>, tar: &mut Vec<i32>) {
    // 若src只剩下一个圆盘，则直接将其移动到tar
    if i == 1 {
        move_pan(src, tar);
        return;
    }
    // 子问题f(i-1):将src顶部i-1个圆盘借助tar移到buf
    dfs(i - 1, src, tar, buf);
    // 子问题f(1):将src剩余一个圆盘移动到tar
    move_pan(src, tar);
    // 子问题f(i-1):将buf顶部i-1个圆盘移动到tar
    dfs(i - 1, buf, src, tar);
}

// 求解汉诺塔问题
fn solve_hanota(A: &mut Vec<i32>, B: &mut Vec<i32>, C: &mut Vec<i32>) {
    let n = A.len() as i32;
    dfs(n, A, B, C);
}

fn main() {
    let mut A = vec![5, 4, 3, 2, 1];
    let mut B = Vec::new();
    let mut C = Vec::new();
    println!("初始状态下：A = {:?}\n B = {:?}\n C = {:?}", A, B, C);

    solve_hanota(&mut A, &mut B, &mut C);

    println!("圆盘移动完成后：A = {:?}\n B = {:?}\n C = {:?}", A, B, C);
}
