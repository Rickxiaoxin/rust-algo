#![allow(unused)]

// 基于邻接矩阵实现的无向图类型
pub struct GraphAdjMat {
    // 顶点列表，元素代表“顶点值”，索引代表“顶点索引”
    pub vertices: Vec<i32>,
    // 邻接矩阵，行列索引对应顶点索引
    pub adj_mat: Vec<Vec<i32>>,
}

impl GraphAdjMat {
    // 构造方法
    pub fn new(vertices: Vec<i32>, edges: Vec<[usize; 2]>) -> Self {
        let mut graph = GraphAdjMat {
            vertices: vec![],
            adj_mat: vec![],
        };
        // 添加顶点
        for val in vertices {
            graph.add_vertex(val);
        }
        // 添加边
        for edge in edges {
            graph.add_edge(edge[0], edge[1]);
        }

        graph
    }

    // 获取顶点数量
    pub fn size(&self) -> usize {
        self.vertices.len()
    }

    // 添加顶点
    pub fn add_vertex(&mut self, val: i32) {
        let n = self.size();
        // 向顶点列表中添加新顶点的值
        self.vertices.push(val);
        // 在邻接矩阵中添加一行
        self.adj_mat.push(vec![0; n]);
        // 在邻接矩阵中添加一列
        for row in &mut self.adj_mat {
            row.push(0);
        }
    }

    // 删除顶点
    pub fn remove_vertex(&mut self, index: usize) {
        if index >= self.size() {
            panic!("index error")
        }
        // 在顶点列表中移除索引index的顶点
        self.vertices.remove(index);
        // 在邻接矩阵中删除索引index的行
        self.adj_mat.remove(index);
        // 在邻接矩阵中删除索引index的列
        for row in &mut self.adj_mat {
            row.remove(index);
        }
    }

    // 添加边
    pub fn add_edge(&mut self, i: usize, j: usize) {
        if i >= self.size() || j >= self.size() {
            panic!("index error")
        }
        // 在无向图中，邻接矩阵关于主对角线对称
        self.adj_mat[i][j] = 1;
        self.adj_mat[j][i] = 1;
    }

    // 删除边
    // 参数i,j对应vertices元素索引
    pub fn remove_edge(&mut self, i: usize, j: usize) {
        if i >= self.size() || j >= self.size() {
            panic!("index error")
        }
        self.adj_mat[i][j] = 0;
        self.adj_mat[j][i] = 0;
    }

    // 打印邻接矩阵
    pub fn print(&self) {
        println!("顶点列表 = {:?}", self.vertices);
        println!("邻接矩阵 = ");
        println!("[");
        for row in &self.adj_mat {
            println!(" {:?},", row);
        }
        println!("]");
    }
}

fn main() {
    // 初始化无向图
    let vertices = vec![1, 3, 2, 5, 4];
    let edges = vec![[0, 1], [0, 3], [1, 2], [2, 3], [2, 4], [3, 4]];
    let mut graph = GraphAdjMat::new(vertices, edges);
    println!("\n初始化后，图为");
    graph.print();

    // 添加边
    // 顶点1,2的索引分别为0,2
    graph.add_edge(0, 2);
    println!("\n添加边1-2后，图为");
    graph.print();

    // 删除边
    // 顶点1,3的索引分别为0,1
    graph.remove_edge(0, 1);
    println!("\n删除边1-3后，图为");
    graph.print();

    // 添加顶点
    graph.add_vertex(6);
    println!("\n添加顶点6后，图为");
    graph.print();

    // 删除顶点
    // 顶点3的索引为1
    graph.remove_vertex(1);
    println!("\n删除顶点3后，图为");
    graph.print();
}
