// 回溯算法：子集和 I
fn backtrack(
    mut state: Vec<i32>,
    target: i32,
    choices: &[i32],
    start: usize,
    res: &mut Vec<Vec<i32>>,
) {
    // 子集和等于target时，记录解
    if target == 0 {
        res.push(state);
        return;
    }
    // 遍历所有选择
    // 剪枝二：从start开始遍历，避免重复子集
    for i in start..choices.len() {
        // 剪枝一：若子集和超过target,则直接结束循环
        // 因为数组已排序，后边元素更大，无需再遍历
        if target - choices[i] < 0 {
            break;
        }
        // 尝试：做出选择，更新target,start
        state.push(choices[i]);
        // 进行下一轮选择
        backtrack(state.clone(), target - choices[i], choices, i, res);
        // 回退：撤销选择，恢复到之前的状态
        state.pop();
    }
}

// 求解子集和 I
fn subset_sum_i(nums: &mut [i32], target: i32) -> Vec<Vec<i32>> {
    let state = Vec::new(); //状态（子集）
    nums.sort();
    let start = 0; // 遍历起始点
    let mut res = Vec::new(); // 结果列表
    backtrack(state, target, nums, start, &mut res);
    res
}

fn main() {
    let mut nums = [3, 4, 5];
    let target = 9;

    let res = subset_sum_i(&mut nums, target);

    println!("输入数组 nums = {:?},target={}", &nums, target);
    println!("所有和等于 {} 的子集 res = {:?}", target, &res);
}
