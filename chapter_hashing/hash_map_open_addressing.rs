#![allow(non_snake_case)]
#![allow(unused)]

#[derive(Debug, Clone, PartialEq)]
pub struct Pair {
    pub key: i32,
    pub val: String,
}

// 开放地址哈希表
struct HashMapOpenAddressing {
    size: usize,                // 键值对数量
    capacity: usize,            // 哈希表容量
    load_thres: f64,            // 负载因子阈值
    extend_ratio: usize,        // 扩容倍数
    buckets: Vec<Option<Pair>>, //桶数组
    TOMBSTONE: Option<Pair>,    //删除标记
}

impl HashMapOpenAddressing {
    // 构造方法
    fn new() -> Self {
        Self {
            size: 0,
            capacity: 4,
            load_thres: 2.0 / 3.0,
            extend_ratio: 2,
            buckets: vec![None; 4],
            TOMBSTONE: Some(Pair {
                key: -1,
                val: "-1".to_string(),
            }),
        }
    }

    // 哈希函数
    fn hash_func(&self, key: i32) -> usize {
        (key % self.capacity as i32) as usize
    }

    // 负载因子
    fn load_factor(&self) -> f64 {
        self.size as f64 / self.capacity as f64
    }

    // 搜索key对应的桶索引
    fn find_bucket(&mut self, key: i32) -> usize {
        let mut index = self.hash_func(key);
        let mut first_tombstone = -1;
    }
}

fn main() {}
