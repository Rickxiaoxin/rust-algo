#[derive(Clone)]
// 键值对
struct Pair {
    key: i32,
    val: String,
}

// 链式地址哈希表
struct HashMapChaining {
    size: i32,
    capacity: i32,
    load_thres: f32,
    extend_ratio: i32,
    buckets: Vec<Vec<Pair>>,
}

impl HashMapChaining {
    // 构造函数
    fn new() -> Self {
        Self {
            size: 0,
            capacity: 4,
            load_thres: 2.0 / 3.0,
            extend_ratio: 2,
            buckets: vec![vec![]; 4],
        }
    }

    // 哈希函数
    fn hash_func(&self, key: i32) -> usize {
        key as usize % self.capacity as usize
    }

    // 负载因子
    fn load_factor(&self) -> f32 {
        self.size as f32 / self.capacity as f32
    }

    // 删除操作
    fn remove(&mut self, key: i32) -> Option<String> {
        let index = self.hash_func(key);
        let bucket = &mut self.buckets[index];

        // 遍历桶，从中删除键对值
        for i in 0..bucket.len() {
            if bucket[i].key == key {
                let pair = bucket.remove(i);
                self.size -= 1;
                return Some(pair.val);
            }
        }
        // 若未找到key，则返回None
        None
    }

    // 扩容哈希表
    fn extend(&mut self) {
        // 暂存原哈希表
        let buckets_tmp = std::mem::replace(&mut self.buckets, vec![]);

        // 初始化扩容后的新哈希表
        self.capacity *= self.extend_ratio;
        self.buckets = vec![Vec::new(); self.capacity as usize];
        self.size = 0;

        // 将键值对从原哈希表搬运至新哈希表
        for bucket in buckets_tmp {
            for pair in bucket {
                self.put(pair.key, pair.val);
            }
        }
    }

    // 打印哈希表
    fn print(&self) {
        for bucket in &self.buckets {
            let mut res = Vec::new();
            for pair in bucket {
                res.push(format!("{} -> {}", pair.key, pair.val));
            }
            println!("{:?}", res);
        }
    }

    // 添加操作
    fn put(&mut self, key: i32, val: String) {
        // 当负载因子超过阈值时，执行扩容
        if self.load_factor() > self.load_thres {
            self.extend();
        }

        let index = self.hash_func(key);
        let bucket = &mut self.buckets[index];

        // 遍历桶，若遇到指定 key,则更新对应val并返回
        for i in 0..bucket.len() {
            if bucket[i].key == key {
                bucket[i].val = val;
                return;
            }
        }

        // 若未找到key，则插入新键值对
        let pair = Pair { key, val };
        bucket.push(pair);
        self.size += 1;
    }

    // 查询操作
    fn get(&self, key: i32) -> Option<&str> {
        let index = self.hash_func(key);
        let bucket = &self.buckets[index];

        // 遍历桶，若找到key，则返回对应val
        for pair in bucket {
            if pair.key == key {
                return Some(&pair.val);
            }
        }
        None
    }
}

fn main() {
    // 初始化哈希表
    let mut map = HashMapChaining::new();

    // 添加操作
    map.put(12836, "小哈".to_string());
    map.put(15937, "小啰".to_string());
    map.put(16750, "小算".to_string());
    map.put(13276, "小法".to_string());
    map.put(10583, "小鸭".to_string());
    println!("添加完成后，哈希表为\nKey -> Value");
    map.print();

    // 查询操作
    println!(
        "\n输入学号 13276，查询得到姓名{}",
        match map.get(13276) {
            Some(value) => value,
            None => "Not a valid Key",
        }
    );

    // 删除操作
    map.remove(12836);
    println!("\n删除12836后，哈希表为\nKey -> Value");
    map.print();
}
