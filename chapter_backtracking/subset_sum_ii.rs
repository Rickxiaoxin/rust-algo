// 回溯算法：子集和 II
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
    // 剪枝三：从start开始遍历，避免重复选择相同元素
    for i in start..choices.len() {
        // 剪枝一：若子集和超过target,则直接结束循环
        if target - choices[i] < 0 {
            break;
        }
        // 剪枝四：如果该元素与左边元素相等，则跳过
        if i > start && choices[i] == choices[i - 1] {
            continue;
        }
        // 尝试：做出选择，更新target,start
        state.push(choices[i]);
        // 进行下一轮选择
        backtrack(state.clone(), target - choices[i], choices, i + 1, res);
        // 回退：撤销选择，恢复到之前的状态
        state.pop();
    }
}

// 求解子集和 II
fn subset_sum_ii(nums: &mut [i32], target: i32) -> Vec<Vec<i32>> {
    let state = Vec::new();
    nums.sort();
    let start = 0;
    let mut res = Vec::new();
    backtrack(state, target, &nums, start, &mut res);
    res
}

fn main() {
    let mut nums = [2, 2, 4, 4, 5, 7];
    let target = 9;

    let res = subset_sum_ii(&mut nums, target);

    println!("输入数组 nums = {:?}, target = {}", &nums, target);
    println!("所有和等于 {} 的子集 res = {:?}", target, &res);
}
