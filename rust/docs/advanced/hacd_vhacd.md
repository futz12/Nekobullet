# HACD / VHACD

HACD（Hierarchical Approximate Convex Decomposition）和 VHACD（Volumetric HACD）用于将凹网格分解为多个凸部件，以便用于碰撞检测。

## HACDParams

HACD 参数配置。

```rust
use nekobullet::*;

let params = HACDParams {
    max_hull_count: 32,           // 最大凸包数量
    max_vertices_per_hull: 64,    // 每个凸包的最大顶点数
    concavity: 0.0025,            // 凹度阈值
    alpha: 0.05,                  // alpha 参数
    beta: 0.05,                   // beta 参数
    cc_connect_dist: 30.0,        // 连接距离
    add_faces_points: true,       // 添加面点
    add_extra_dist_points: false, // 添加额外距离点
    add_neighbours_dist_points: false, // 添加邻居距离点
};

// 使用默认值
let params = HACDParams::default();
```

## HACD

HACD 分解器。

### 创建和设置

```rust
let mut hacd = HACD::new();

// 设置参数
hacd.set_params(&params);

// 获取参数
let current_params = hacd.get_params();
```

### 设置网格

```rust
// 设置顶点（每3个值为一个顶点）
let points: Vec<f64> = vec![
    0.0, 0.0, 0.0,
    1.0, 0.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 0.0, 1.0,
];

// 设置三角形索引（每3个值为一个三角形）
let triangles: Vec<i32> = vec![
    0, 1, 2,
    0, 2, 3,
    0, 3, 1,
    1, 3, 2,
];

hacd.set_mesh(&points, &triangles).expect("Failed to set mesh");
```

### 计算

```rust
// 执行分解
hacd.compute().expect("HACD computation failed");

// 获取凸包数量
let num_hulls = hacd.get_num_hulls();
println!("Generated {} convex hulls", num_hulls);
```

### 获取结果

```rust
// 获取单个凸包
if let Some(hull) = hacd.get_hull(0) {
    println!("Hull 0: {} points, {} triangles",
             hull.num_points(), hull.num_triangles());
    
    // 获取顶点
    for i in 0..hull.num_points() {
        let point = hull.get_point(i);
        println!("  Point {}: {:?}", i, point);
    }
    
    // 获取三角形
    for i in 0..hull.num_triangles() {
        let tri = hull.get_triangle(i);
        println!("  Triangle {}: {:?}", i, tri);
    }
}

// 获取所有凸包
let hulls = hacd.get_hulls();
```

## VHACDParams

VHACD 参数配置（改进版本）。

```rust
use nekobullet::*;

let params = VHACDParams {
    resolution: 100000,           // 体素分辨率
    depth: 20,                    // 递归深度
    concavity: 0.0025,            // 凹度阈值
    alpha: 0.05,                  // alpha 参数
    beta: 0.05,                   // beta 参数
    gamma: 0.00125,               // gamma 参数
    max_vertices_per_hull: 64,    // 每个凸包的最大顶点数
    min_volume_per_ch: 0.0001,    // 每个凸包的最小体积
    plane_downsampling: 4,        // 平面下采样
    convexhull_downsampling: 4,   // 凸包下采样
    pca: false,                   // PCA
    mode: 0,                      // 模式
    convexhull_approximation: true, // 凸包近似
};

// 使用默认值
let params = VHACDParams::default();
```

## VHACD

VHACD 分解器（改进版本）。

### 创建和计算

```rust
let mut vhacd = VHACD::new();

// 一步计算
let points: Vec<f64> = vec![/* ... */];
let triangles: Vec<i32> = vec![/* ... */];

if vhacd.compute(&points, &triangles, &params) {
    println!("VHACD succeeded");
}
```

### 取消计算

```rust
// 取消正在进行的计算
vhacd.cancel();
```

### 获取结果

```rust
// 获取凸包数量
let num_hulls = vhacd.get_num_hulls();

// 获取单个凸包
if let Some(hull) = vhacd.get_hull(0) {
    println!("Hull 0: {} points, {} triangles",
             hull.num_points(), hull.num_triangles());
}

// 获取所有凸包
let hulls = vhacd.get_hulls();
```

### 清理

```rust
vhacd.clean();
```

## ConvexHull

凸包结果。

```rust
pub struct ConvexHull {
    pub points: Vec<f64>,     // 顶点（每3个值为一个顶点）
    pub triangles: Vec<i32>,  // 三角形索引
}

impl ConvexHull {
    pub fn num_points(&self) -> i32;
    pub fn num_triangles(&self) -> i32;
    pub fn get_point(&self, index: i32) -> [f64; 3];
    pub fn get_triangle(&self, index: i32) -> [i32; 3];
}
```

## 完整示例

### 模型预处理

```rust
use nekobullet::*;

fn decompose_model(vertices: &[f64], triangles: &[i32]) -> Vec<ShapeHandle> {
    let params = VHACDParams {
        resolution: 50000,
        max_vertices_per_hull: 32,
        ..Default::default()
    };

    let mut vhacd = VHACD::new();
    
    if !vhacd.compute(vertices, triangles, &params) {
        println!("VHACD failed");
        return Vec::new();
    }

    let mut shapes = Vec::new();
    let hulls = vhacd.get_hulls();

    for hull in hulls {
        // 将 f64 转换为 f32
        let points: Vec<Vec3> = (0..hull.num_points())
            .map(|i| {
                let p = hull.get_point(i);
                Vec3::new(p[0] as f32, p[1] as f32, p[2] as f32)
            })
            .collect();

        // 创建凸包形状
        let shape = ShapeHandle::new_convex_hull(&points);
        shapes.push(shape);
    }

    shapes
}

fn main() {
    // 示例：创建一个 L 形模型
    let vertices: Vec<f64> = vec![
        // 底部
        0.0, 0.0, 0.0,
        2.0, 0.0, 0.0,
        2.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        // 顶部
        0.0, 2.0, 0.0,
        1.0, 2.0, 0.0,
        1.0, 2.0, 1.0,
        0.0, 2.0, 1.0,
    ];

    let triangles: Vec<i32> = vec![
        0, 1, 2, 0, 2, 3,  // 底面
        4, 6, 5, 4, 7, 6,  // 顶面
        0, 4, 1, 1, 4, 5,  // 前面
        1, 5, 2, 2, 5, 6,  // 右面
        2, 6, 3, 3, 6, 7,  // 后面
        3, 7, 0, 0, 7, 4,  // 左面
    ];

    let shapes = decompose_model(&vertices, &triangles);
    println!("Generated {} convex shapes", shapes.len());

    // 创建复合形状
    let mut compound = ShapeHandle::new_compound();
    for shape in shapes {
        compound.add_child_shape(shape, Transform::identity());
    }
}
```

### 创建碰撞物体

```rust
use nekobullet::*;

fn create_collision_object(vertices: &[f64], triangles: &[i32]) -> Option<ShapeHandle> {
    let params = VHACDParams::default();
    let mut vhacd = VHACD::new();

    if !vhacd.compute(vertices, triangles, &params) {
        return None;
    }

    let hulls = vhacd.get_hulls();
    if hulls.is_empty() {
        return None;
    }

    if hulls.len() == 1 {
        // 单个凸包
        let hull = &hulls[0];
        let points: Vec<Vec3> = (0..hull.num_points())
            .map(|i| {
                let p = hull.get_point(i);
                Vec3::new(p[0] as f32, p[1] as f32, p[2] as f32)
            })
            .collect();
        Some(ShapeHandle::new_convex_hull(&points))
    } else {
        // 复合形状
        let mut compound = ShapeHandle::new_compound();
        for hull in hulls {
            let points: Vec<Vec3> = (0..hull.num_points())
                .map(|i| {
                    let p = hull.get_point(i);
                    Vec3::new(p[0] as f32, p[1] as f32, p[2] as f32)
                })
                .collect();
            let shape = ShapeHandle::new_convex_hull(&points);
            compound.add_child_shape(shape, Transform::identity());
        }
        Some(compound)
    }
}
```

## HACD API

```rust
impl HACD {
    pub fn new() -> Self;
    pub fn set_params(&self, params: &HACDParams);
    pub fn get_params(&self) -> HACDParams;
    pub fn set_mesh(&self, points: &[f64], triangles: &[i32]) -> Result<(), String>;
    pub fn compute(&self) -> Result<(), String>;
    pub fn get_num_hulls(&self) -> i32;
    pub fn get_hull(&self, hull_index: i32) -> Option<ConvexHull>;
    pub fn get_hulls(&self) -> Vec<ConvexHull>;
}
```

## VHACD API

```rust
impl VHACD {
    pub fn new() -> Self;
    pub fn compute(&self, points: &[f64], triangles: &[i32], params: &VHACDParams) -> bool;
    pub fn cancel(&self);
    pub fn get_num_hulls(&self) -> i32;
    pub fn get_hull(&self, index: i32) -> Option<ConvexHull>;
    pub fn get_hulls(&self) -> Vec<ConvexHull>;
    pub fn clean(&self);
}
```

## 性能提示

1. **调整分辨率**：较高的分辨率会产生更精确的结果，但计算时间更长
2. **限制凸包数量**：设置合理的 `max_hull_count`
3. **限制顶点数**：设置合理的 `max_vertices_per_hull`
4. **预处理模型**：在游戏运行前进行分解，保存结果
5. **使用 VHACD**：VHACD 通常产生更好的结果
