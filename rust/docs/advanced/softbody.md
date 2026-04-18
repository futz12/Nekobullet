# SoftBody

软体用于模拟可变形物体，如布料、绳索、弹性物体等。

## SoftBodyWorldInfo

软体世界信息，包含软体模拟的全局设置。

### 创建

```rust
use nekobullet::*;

let world_info = SoftBodyWorldInfo::new();
```

### 设置

```rust
// 设置重力
world_info.set_gravity(Vec3::new(0.0, -9.81, 0.0));

// 设置空气密度
world_info.set_air_density(1.2);

// 设置水密度
world_info.set_water_density(0.0);

// 设置水面偏移
world_info.set_water_offset(0.0);

// 设置水面法线
world_info.set_water_normal(Vec3::new(0.0, 1.0, 0.0));
```

## SoftBody

软体对象。

### 创建绳索

```rust
let rope = SoftBody::create_rope(
    &world_info,
    Vec3::new(0.0, 10.0, 0.0),  // 起点
    Vec3::new(5.0, 10.0, 0.0),  // 终点
    10,   // 分辨率（节点数）
    1,    // 固定点（1=固定起点，2=固定终点，3=固定两端）
);
```

### 创建面片

```rust
let corners = [
    Vec3::new(0.0, 10.0, 0.0),
    Vec3::new(5.0, 10.0, 0.0),
    Vec3::new(0.0, 10.0, 5.0),
    Vec3::new(5.0, 10.0, 5.0),
];

let patch = SoftBody::create_patch(
    &world_info,
    corners,
    10,   // X 分辨率
    10,   // Y 分辨率
    1,    // 固定点
    true, // 生成对角线
);
```

### 创建椭球

```rust
let ellipsoid = SoftBody::create_ellipsoid(
    &world_info,
    Vec3::new(0.0, 5.0, 0.0),  // 中心
    Vec3::new(1.0, 2.0, 1.0),  // 半径
    20,   // 分辨率
);
```

### 从三角形网格创建

```rust
let vertices: Vec<Real> = vec![
    0.0, 0.0, 0.0,
    1.0, 0.0, 0.0,
    0.0, 1.0, 0.0,
];

let triangles: Vec<i32> = vec![0, 1, 2];

let softbody = SoftBody::create_from_trimesh(&world_info, &vertices, &triangles);
```

### 从凸包创建

```rust
let vertices: Vec<Real> = vec![
    0.0, 0.0, 0.0,
    1.0, 0.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 0.0, 1.0,
];

let softbody = SoftBody::create_from_convex_hull(&world_info, &vertices);
```

## 节点操作

### 获取节点信息

```rust
// 获取节点数量
let num_nodes = softbody.num_nodes();

// 获取节点位置
let pos = softbody.get_node_position(0);

// 获取节点速度
let vel = softbody.get_node_velocity(0);

// 获取节点质量
let mass = softbody.get_node_mass(0);
```

### 设置节点属性

```rust
// 设置节点位置
softbody.set_node_position(0, Vec3::new(1.0, 2.0, 3.0));

// 设置节点速度
softbody.set_node_velocity(0, Vec3::new(0.0, 1.0, 0.0));

// 设置节点质量
softbody.set_node_mass(0, 0.1);
```

## 质量操作

```rust
// 获取总质量
let total_mass = softbody.total_mass();

// 设置总质量
softbody.set_total_mass(10.0);
```

## 力和速度

```rust
// 设置整体速度
softbody.set_velocity(Vec3::new(0.0, 1.0, 0.0));

// 添加速度
softbody.add_velocity(Vec3::new(0.0, 0.5, 0.0));

// 施加力
softbody.apply_force(Vec3::new(0.0, 100.0, 0.0));

// 施加冲量
softbody.apply_impulse(Vec3::new(0.0, 10.0, 0.0));

// 清除力
softbody.clear_forces();
```

## 锚点

将软体节点锚定到刚体。

```rust
// 添加锚点
softbody.append_anchor(
    0,  // 节点索引
    &rigid_body,
    Vec3::new(0.0, 0.0, 0.0),  // 局部位置
    false,  // 禁用碰撞
);

// 移除锚点
softbody.remove_anchor(0);
```

## 材质设置

```rust
// 设置材质刚度
// k_lst: 线性刚度
// k_ast: 面积刚度
// k_vst: 体积刚度
softbody.set_material_stiffness(0.5, 0.5, 0.5);
```

## 风力

```rust
// 设置风速
softbody.set_wind_velocity(Vec3::new(10.0, 0.0, 0.0));

// 获取风速
let wind = softbody.wind_velocity();
```

## 配置参数

```rust
// 设置阻尼
softbody.set_config_damping(0.1);

// 设置阻力
softbody.set_config_drag(0.0);

// 设置升力
softbody.set_config_lift(0.0);

// 设置压力
softbody.set_config_pressure(0.0);

// 设置体积守恒
softbody.set_config_volume_conversation(1.0);

// 设置时间缩放
softbody.set_config_time_scale(1.0);
```

## 完整示例

### 布料模拟

```rust
use nekobullet::*;

fn main() {
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 创建软体世界信息
    let world_info = SoftBodyWorldInfo::new();
    world_info.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 创建布料
    let corners = [
        Vec3::new(-2.0, 10.0, -2.0),
        Vec3::new(2.0, 10.0, -2.0),
        Vec3::new(-2.0, 10.0, 2.0),
        Vec3::new(2.0, 10.0, 2.0),
    ];

    let mut cloth = SoftBody::create_patch(
        &world_info,
        corners,
        20,   // X 分辨率
        20,   // Y 分辨率
        3,    // 固定四角
        true,
    );

    // 设置材质
    cloth.set_material_stiffness(1.0, 1.0, 0.0);
    cloth.set_config_damping(0.01);

    // 添加到世界
    world.add_softbody(&cloth);

    // 模拟
    for i in 0..240 {
        world.step(1.0 / 60.0);
        
        // 获取中心节点位置
        let center = cloth.get_node_position(cloth.num_nodes() / 2);
        println!("Frame {}: center y = {:.2}", i, center.y);
    }
}
```

### 绳索模拟

```rust
use nekobullet::*;

fn main() {
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 创建软体世界信息
    let world_info = SoftBodyWorldInfo::new();
    world_info.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 创建绳索
    let rope = SoftBody::create_rope(
        &world_info,
        Vec3::new(0.0, 10.0, 0.0),  // 起点
        Vec3::new(5.0, 10.0, 0.0),  // 终点
        20,   // 节点数
        1,    // 固定起点
    );

    world.add_softbody(&rope);

    // 模拟
    for i in 0..240 {
        world.step(1.0 / 60.0);
        
        // 获取末端节点位置
        let end = rope.get_node_position(rope.num_nodes() - 1);
        println!("Frame {}: end pos = ({:.2}, {:.2}, {:.2})", 
                 i, end.x, end.y, end.z);
    }
}
```

### 弹性球

```rust
use nekobullet::*;

fn main() {
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 创建软体世界信息
    let world_info = SoftBodyWorldInfo::new();
    world_info.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 创建弹性球
    let ball = SoftBody::create_ellipsoid(
        &world_info,
        Vec3::new(0.0, 5.0, 0.0),  // 中心
        Vec3::new(0.5, 0.5, 0.5),  // 半径
        16,   // 分辨率
    );

    // 设置弹性
    ball.set_material_stiffness(0.8, 0.8, 0.8);
    ball.set_total_mass(1.0);

    world.add_softbody(&ball);

    // 模拟
    for i in 0..240 {
        world.step(1.0 / 60.0);
        
        let center = ball.get_node_position(0);
        println!("Frame {}: y = {:.2}", i, center.y);
    }
}
```

## 性能提示

1. **减少节点数量**：节点越多，计算开销越大
2. **使用适当的分辨率**：根据需要选择合适的分辨率
3. **设置合理的刚度**：过高的刚度可能导致不稳定
4. **使用锚点**：将软体固定到刚体可以提高稳定性
