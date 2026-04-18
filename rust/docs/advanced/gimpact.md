# GImpact

GImpact 形状用于动态三角形网格的碰撞检测，支持凹网格的精确碰撞。

## GImpactShape

GImpact 形状，用于动态凹网格碰撞。

### 创建

```rust
use nekobullet::*;

// 首先创建三角形网格形状
let vertices = vec![
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(1.0, 1.0, 0.0),
];

let indices = vec![0, 1, 2, 1, 3, 2];
let trimesh = ShapeHandle::new_triangle_mesh(&vertices, &indices);

// 从三角形网格创建 GImpact 形状
let gimpact = unsafe { GImpactShape::from_shape_handle(
    &trimesh,
    Vec3::new(1.0, 1.0, 1.0),  // 缩放
) };
```

### 属性

```rust
// 更新边界
gimpact.update_bound();

// 获取子形状数量
let num_children = gimpact.num_child_shapes();

// 设置边距
gimpact.set_margin(0.04);

// 获取边距
let margin = gimpact.get_margin();

// 获取句柄
let handle = gimpact.handle();
```

## GImpactDecompShape

GImpact 分解形状，将凹网格分解为多个凸形状以提高性能。

### 创建

```rust
use nekobullet::*;

let trimesh = ShapeHandle::new_triangle_mesh(&vertices, &indices);

let decomp = GImpactDecompShape::new(
    &trimesh,
    Vec3::new(1.0, 1.0, 1.0),  // 缩放
    0.04,   // 边距
    true,   // 变换子形状
).expect("Failed to create GImpactDecompShape");
```

### 属性

```rust
// 获取子形状数量
let num_children = decomp.num_child_shapes();

// 更新边界
decomp.update_bound();

// 设置边距
decomp.set_margin(0.04);

// 获取边距
let margin = decomp.get_margin();

// 获取子形状变换
if let Some(transform) = decomp.get_child_transform(0) {
    println!("Child 0 position: {:?}", transform.position);
    println!("Child 0 rotation: {:?}", transform.rotation);
}

// 获取子形状原生指针（非 owning）
if let Some(child_ptr) = decomp.get_child_shape_ptr(0) {
    println!("Child 0 ptr: {:?}", child_ptr);
}
```

## 完整示例

### 动态凹网格

```rust
use nekobullet::*;

fn main() {
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 创建地面
    let ground_shape = ShapeHandle::new_box(Vec3::new(50.0, 0.5, 50.0));
    let ground = RigidBodyBuilder::new()
        .shape(ground_shape)
        .mass(0.0)
        .build()
        .unwrap();
    world.add_rigid_body(ground);

    // 创建凹网格（例如一个碗形）
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    let segments = 16;
    let radius = 2.0;
    let depth = 1.0;

    // 创建碗的顶点
    for i in 0..=segments {
        let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        vertices.push(Vec3::new(x, 0.0, z));      // 边缘
        vertices.push(Vec3::new(x * 0.5, -depth, z * 0.5)); // 底部
    }

    // 创建三角形索引
    for i in 0..segments {
        let base = i * 2;
        let next = ((i + 1) % (segments + 1)) * 2;
        
        indices.push(base as i32);
        indices.push((base + 1) as i32);
        indices.push(next as i32);
        
        indices.push((base + 1) as i32);
        indices.push((next + 1) as i32);
        indices.push(next as i32);
    }

    // 创建三角形网格
    let trimesh = ShapeHandle::new_triangle_mesh(&vertices, &indices);

    // 创建 GImpact 形状
    let gimpact = unsafe { GImpactShape::from_shape_handle(&trimesh, Vec3::ONE) };

    // 创建刚体（注意：GImpact 形状需要特殊处理）
    // 通常使用复合形状包装

    // 创建球来测试碰撞
    let ball_shape = ShapeHandle::new_sphere(0.5);
    let ball = RigidBodyBuilder::new()
        .shape(ball_shape)
        .mass(1.0)
        .position(Vec3::new(0.0, 5.0, 0.0))
        .build()
        .unwrap();
    world.add_rigid_body(ball);

    // 模拟
    for i in 0..120 {
        world.step(1.0 / 60.0);
    }
}
```

### 使用分解形状

```rust
use nekobullet::*;

fn main() {
    // 创建复杂的凹网格
    let vertices = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 2.0),
        Vec3::new(0.0, 0.0, 2.0),
        Vec3::new(0.5, 1.0, 0.5),
        Vec3::new(1.5, 1.0, 0.5),
        Vec3::new(1.5, 1.0, 1.5),
        Vec3::new(0.5, 1.0, 1.5),
    ];

    let indices = vec![
        // 底面
        0, 1, 2, 0, 2, 3,
        // 顶面
        4, 6, 5, 4, 7, 6,
        // 侧面
        0, 4, 1, 1, 4, 5,
        1, 5, 2, 2, 5, 6,
        2, 6, 3, 3, 6, 7,
        3, 7, 0, 0, 7, 4,
    ];

    let trimesh = ShapeHandle::new_triangle_mesh(&vertices, &indices);

    // 创建分解形状
    let decomp = GImpactDecompShape::new(
        &trimesh,
        Vec3::ONE,
        0.01,
        true,
    ).expect("Failed to create decomp shape");

    println!("Created {} child shapes", decomp.num_child_shapes());

    // 输出子形状变换
    for i in 0..decomp.num_child_shapes() {
        if let Some(transform) = decomp.get_child_transform(i) {
            println!("Child {}: pos={:?}, rot={:?}",
                     i, transform.position, transform.rotation);
        }
    }
}
```

## GImpactShape API

```rust
impl GImpactShape {
    pub unsafe fn from_trimesh(trimesh_handle: *mut c_void, scale: Vec3) -> Self;
    pub unsafe fn from_shape_handle(trimesh: &ShapeHandle, scale: Vec3) -> Self;
    
    pub fn update_bound(&mut self);
    pub fn num_child_shapes(&self) -> i32;
    pub fn set_margin(&mut self, margin: Real);
    pub fn get_margin(&self) -> Real;
    pub fn handle(&self) -> *mut c_void;
}
```

## GImpactDecompShape API

```rust
impl GImpactDecompShape {
    pub fn new(
        trimesh: &ShapeHandle,
        scale: Vec3,
        margin: Real,
        transform_subshapes: bool,
    ) -> Option<Self>;
    
    pub fn handle(&self) -> *mut c_void;
    pub fn num_child_shapes(&self) -> i32;
    pub fn update_bound(&self);
    pub fn set_margin(&self, margin: Real);
    pub fn get_margin(&self) -> Real;
    pub fn get_child_transform(&self, index: i32) -> Option<Transform>;
}
```

## 性能提示

1. **使用分解形状**：对于复杂网格，使用 GImpactDecompShape 可以提高性能
2. **合理设置边距**：较大的边距可以提高稳定性
3. **避免过多三角形**：减少网格复杂度
4. **静态网格使用 TriangleMesh**：静态物体使用普通三角形网格更快
5. **更新边界**：变形后记得调用 `update_bound()`
