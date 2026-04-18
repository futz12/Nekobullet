# CollisionShape

碰撞形状定义了物体的碰撞边界，是物理模拟的基础。

## 形状类型

### 基本形状

| 形状 | 描述 | 适用场景 |
|------|------|----------|
| Box | 盒子形状 | 建筑物、箱子、平台 |
| Sphere | 球体形状 | 球、弹丸、简单物体 |
| Capsule | 胶囊形状 | 角色、圆柱形物体 |
| Cylinder | 圆柱形状 | 罐子、柱子 |
| Cone | 圆锥形状 | 锥形物体 |
| Plane | 无限平面 | 地面 |

### 复杂形状

| 形状 | 描述 | 适用场景 |
|------|------|----------|
| ConvexHull | 凸包 | 复杂凸物体 |
| Compound | 复合形状 | 多部件物体 |
| TriangleMesh | 三角形网格 | 静态复杂几何 |
| Heightfield | 高度场 | 地形 |

## 创建形状

### 使用 CollisionShapeBuilder

```rust
use nekobullet::*;

// 盒子形状
let box_shape = CollisionShapeBuilder::new()
    .box_shape(Vec3::new(1.0, 2.0, 1.0))  // 半尺寸
    .build()
    .unwrap();

// 球体形状
let sphere_shape = CollisionShapeBuilder::new()
    .sphere(0.5)  // 半径
    .build()
    .unwrap();

// 胶囊形状
let capsule_shape = CollisionShapeBuilder::new()
    .capsule(0.5, 1.0)  // 半径, 高度
    .build()
    .unwrap();

// 圆柱形状
let cylinder_shape = CollisionShapeBuilder::new()
    .cylinder(Vec3::new(0.5, 1.0, 0.5))  // 半尺寸
    .build()
    .unwrap();

// 圆锥形状
let cone_shape = CollisionShapeBuilder::new()
    .cone(0.5, 1.0)  // 半径, 高度
    .build()
    .unwrap();

// 平面形状
let plane_shape = CollisionShapeBuilder::new()
    .plane(Vec3::new(0.0, 1.0, 0.0), 0.0)  // 法线, 常数
    .build()
    .unwrap();
```

### 使用 ShapeHandle 直接创建

```rust
use nekobullet::*;

// 盒子
let box_shape = ShapeHandle::new_box(Vec3::new(1.0, 1.0, 1.0));

// 球体
let sphere_shape = ShapeHandle::new_sphere(0.5);

// 胶囊
let capsule_shape = ShapeHandle::new_capsule(0.5, 1.0);

// 圆柱
let cylinder_shape = ShapeHandle::new_cylinder(Vec3::new(0.5, 1.0, 0.5));

// 圆锥
let cone_shape = ShapeHandle::new_cone(0.5, 1.0);

// 平面
let plane_shape = ShapeHandle::new_plane(Vec3::new(0.0, 1.0, 0.0), 0.0);
```

## 凸包形状

```rust
// 从点集创建凸包
let points = vec![
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(1.0, 1.0, 0.0),
    Vec3::new(1.0, 0.0, 1.0),
    Vec3::new(0.0, 1.0, 1.0),
    Vec3::new(1.0, 1.0, 1.0),
];

let convex_hull = ShapeHandle::new_convex_hull(&points);

// 使用构建器
let convex_hull = CollisionShapeBuilder::new()
    .convex_hull(points)
    .build()
    .unwrap();
```

## 复合形状

```rust
// 创建复合形状
let mut compound = ShapeHandle::new_compound();

// 添加子形状
let box1 = ShapeHandle::new_box(Vec3::new(1.0, 0.5, 1.0));
let box2 = ShapeHandle::new_box(Vec3::new(0.5, 1.0, 0.5));

compound.add_child_shape(
    box1,
    Transform::from_position(Vec3::new(0.0, 0.0, 0.0)),
);

compound.add_child_shape(
    box2,
    Transform::from_position(Vec3::new(0.0, 1.5, 0.0)),
);
```

## 三角形网格形状

```rust
// 创建三角形网格（用于静态物体）
let vertices = vec![
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(1.0, 1.0, 0.0),
];

let indices = vec![
    0, 1, 2,  // 第一个三角形
    1, 3, 2,  // 第二个三角形
];

let trimesh = ShapeHandle::new_triangle_mesh(&vertices, &indices);

// 使用构建器
let trimesh = CollisionShapeBuilder::new()
    .triangle_mesh(vertices, indices)
    .build()
    .unwrap();
```

## 高度场形状

```rust
// 创建高度场（用于地形）
let width = 100;
let length = 100;
let heights: Vec<Real> = (0..width * length)
    .map(|i| (i as f32 * 0.1).sin())
    .collect();

let heightfield = ShapeHandle::new_heightfield(
    width as i32,
    length as i32,
    &heights,
    -1.0,   // 最小高度
    1.0,    // 最大高度
    1,      // 上轴 (Y)
);

// 使用构建器
let heightfield = CollisionShapeBuilder::new()
    .heightfield(width as i32, length as i32, heights, -1.0, 1.0, 1)
    .build()
    .unwrap();
```

## 形状属性

### 类型

```rust
let shape_type = shape.shape_type();

match shape_type {
    CollisionShapeType::Box => println!("Box shape"),
    CollisionShapeType::Sphere => println!("Sphere shape"),
    CollisionShapeType::Capsule => println!("Capsule shape"),
    // ...
}
```

### 缩放

```rust
// 设置局部缩放
shape.set_local_scaling(Vec3::new(2.0, 1.0, 2.0));

// 获取局部缩放
let scale = shape.get_local_scaling();
```

### 边距

```rust
// 设置碰撞边距
shape.set_margin(0.04);

// 获取碰撞边距
let margin = shape.get_margin();
```

### 惯性计算

```rust
// 计算局部惯性张量
let mass = 1.0;
let inertia = shape.calculate_local_inertia(mass);
println!("Inertia: {:?}", inertia);
```

### 运动特征

```rust
// 获取角运动判别式
let disc = shape.get_angular_motion_disc();

// 获取接触断开阈值
let threshold = shape.get_contact_breaking_threshold(0.02);
```

## 形状特定属性

### 盒子

```rust
let half_extents = box_shape.get_box_half_extents();
```

### 球体

```rust
let radius = sphere_shape.get_sphere_radius();
```

### 胶囊

```rust
let radius = capsule_shape.get_capsule_radius();
let half_height = capsule_shape.get_half_height();
```

### 圆柱

```rust
let radius = cylinder_shape.get_cylinder_radius();
let half_height = cylinder_shape.get_cylinder_half_height();
```

### 圆锥

```rust
let radius = cone_shape.get_cone_radius();
let height = cone_shape.get_cone_height();
```

## CollisionShapeBuilder 完整 API

```rust
impl CollisionShapeBuilder {
    pub fn new() -> Self;
    
    // 基本形状
    pub fn box_shape(self, half_extents: Vec3) -> Self;
    pub fn sphere(self, radius: Real) -> Self;
    pub fn capsule(self, radius: Real, height: Real) -> Self;
    pub fn cylinder(self, half_extents: Vec3) -> Self;
    pub fn cone(self, radius: Real, height: Real) -> Self;
    pub fn plane(self, normal: Vec3, constant: Real) -> Self;
    
    // 复杂形状
    pub fn convex_hull(self, points: Vec<Vec3>) -> Self;
    pub fn triangle_mesh(self, vertices: Vec<Vec3>, indices: Vec<i32>) -> Self;
    pub fn heightfield(
        self,
        width: i32,
        length: i32,
        heights: Vec<Real>,
        min_height: Real,
        max_height: Real,
        up_axis: i32,
    ) -> Self;
    
    // 构建
    pub fn build(self) -> Option<ShapeHandle>;
}
```

## ShapeHandle 完整 API

```rust
impl ShapeHandle {
    // 创建方法
    pub fn new_box(half_extents: Vec3) -> Self;
    pub fn new_sphere(radius: Real) -> Self;
    pub fn new_capsule(radius: Real, height: Real) -> Self;
    pub fn new_cylinder(half_extents: Vec3) -> Self;
    pub fn new_cone(radius: Real, height: Real) -> Self;
    pub fn new_plane(normal: Vec3, constant: Real) -> Self;
    pub fn new_convex_hull(points: &[Vec3]) -> Self;
    pub fn new_compound() -> Self;
    pub fn new_triangle_mesh(vertices: &[Vec3], indices: &[i32]) -> Self;
    pub fn new_heightfield(...) -> Self;
    
    // 复合形状
    pub fn add_child_shape(&mut self, child: ShapeHandle, local_transform: Transform);
    
    // 属性
    pub fn shape_type(&self) -> CollisionShapeType;
    pub fn handle(&self) -> *mut c_void;
    
    // 计算
    pub fn calculate_local_inertia(&self, mass: Real) -> Vec3;
    
    // 缩放
    pub fn set_local_scaling(&mut self, scaling: Vec3);
    pub fn get_local_scaling(&self) -> Vec3;
    
    // 边距
    pub fn set_margin(&mut self, margin: Real);
    pub fn get_margin(&self) -> Real;
    
    // 运动特征
    pub fn get_angular_motion_disc(&self) -> Real;
    pub fn get_contact_breaking_threshold(&self, default_threshold: Real) -> Real;
    
    // 形状特定
    pub fn get_box_half_extents(&self) -> Vec3;
    pub fn get_sphere_radius(&self) -> Real;
    pub fn get_capsule_radius(&self) -> Real;
    pub fn get_capsule_half_height(&self) -> Real;
    pub fn get_cylinder_radius(&self) -> Real;
    pub fn get_cylinder_half_height(&self) -> Real;
    pub fn get_cone_radius(&self) -> Real;
    pub fn get_cone_height(&self) -> Real;
}
```

## 完整示例

```rust
use nekobullet::*;

fn main() {
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 创建地面（平面）
    let ground_shape = ShapeHandle::new_plane(Vec3::new(0.0, 1.0, 0.0), 0.0);
    let ground = RigidBodyBuilder::new()
        .shape(ground_shape)
        .mass(0.0)
        .build()
        .unwrap();
    world.add_rigid_body(ground);

    // 创建盒子
    let box_shape = ShapeHandle::new_box(Vec3::new(0.5, 0.5, 0.5));
    let box = RigidBodyBuilder::new()
        .shape(box_shape)
        .mass(1.0)
        .position(Vec3::new(-2.0, 5.0, 0.0))
        .build()
        .unwrap();
    world.add_rigid_body(box);

    // 创建球
    let sphere_shape = ShapeHandle::new_sphere(0.5);
    let sphere = RigidBodyBuilder::new()
        .shape(sphere_shape)
        .mass(1.0)
        .position(Vec3::new(0.0, 5.0, 0.0))
        .restitution(0.8)
        .build()
        .unwrap();
    world.add_rigid_body(sphere);

    // 创建胶囊
    let capsule_shape = ShapeHandle::new_capsule(0.3, 1.0);
    let capsule = RigidBodyBuilder::new()
        .shape(capsule_shape)
        .mass(1.0)
        .position(Vec3::new(2.0, 5.0, 0.0))
        .build()
        .unwrap();
    world.add_rigid_body(capsule);

    // 模拟
    for _ in 0..120 {
        world.step(1.0 / 60.0);
    }
}
```

## 性能提示

1. **使用简单形状**：球体和盒子比凸包和三角形网格更快
2. **静态三角形网格**：三角形网格只应用于静态物体
3. **合理设置边距**：较大的边距可以提高稳定性但降低精度
4. **复合形状优化**：尽量减少复合形状的子形状数量
