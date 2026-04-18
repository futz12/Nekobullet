# PhysicsWorld

物理世界是 Nekobullet 的核心，管理所有物理对象并推进物理模拟。

## 创建

### 基本创建

```rust
use nekobullet::*;

// 创建默认物理世界
let world = PhysicsWorld::new();
```

### 使用构建器

```rust
use nekobullet::*;

let world = PhysicsWorldBuilder::new()
    .gravity(Vec3::new(0.0, -10.0, 0.0))
    .broadphase(BroadphaseType::DynamicAabbTree)
    .solver_iterations(10)
    .build();
```

## 属性设置

### 重力

```rust
// 设置重力
world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

// 获取重力
let gravity = world.get_gravity();
```

### 时间步长

```rust
// 设置时间步长
world.set_time_step(1.0 / 60.0);

// 获取时间步长
let dt = world.get_time_step();

// 设置最大子步数
world.set_max_sub_steps(4);
let max_steps = world.get_max_sub_steps();
```

### 接触阈值

```rust
// 设置接触断开阈值
world.set_contact_breaking_threshold(0.02);

// 获取阈值
let threshold = world.get_contact_breaking_threshold();
```

## 物理模拟

### 步进模拟

```rust
// 简单步进（使用默认参数）
world.step(1.0 / 60.0);

// 完整控制
world.step_simulation(
    1.0 / 60.0,  // 时间步长
    4,           // 最大子步数
    1.0 / 60.0   // 固定时间步长
);
```

### 清除力

```rust
world.clear_forces();
```

## 刚体管理

### 添加刚体

```rust
// 基本添加
let handle = world.add_rigid_body(body);

// 带碰撞过滤添加
let handle = world.add_rigid_body_with_filter(
    body,
    1,   // 碰撞组
    -1   // 碰撞掩码
);
```

### 获取刚体

```rust
// 不可变引用
if let Some(body) = world.get_rigid_body(handle) {
    let pos = body.get_position();
}

// 可变引用
if let Some(body) = world.get_rigid_body_mut(handle) {
    body.apply_force(Vec3::new(0.0, 100.0, 0.0));
}
```

### 移除刚体

```rust
if let Some(body) = world.remove_rigid_body(handle) {
    println!("Removed body");
}
```

### 遍历刚体

```rust
// 不可变遍历
for (handle, body) in world.bodies() {
    println!("Handle {}: {:?}", handle, body.get_position());
}

// 可变遍历
for (handle, body) in world.bodies_mut() {
    body.activate();
}
```

### 统计

```rust
let body_count = world.body_count();
let num_objects = world.get_num_collision_objects();
let num_rigid_bodies = world.get_num_rigid_bodies();
```

## 约束管理

### 创建约束

```rust
// 点对点约束
let constraint = world.create_point2point_constraint(
    handle_a,
    handle_b,
    Vec3::new(0.5, 0.0, 0.0),   // body A 的锚点
    Vec3::new(-0.5, 0.0, 0.0),  // body B 的锚点
);

// 6自由度约束
let constraint = world.create_generic_6dof_constraint(
    handle_a,
    handle_b,
    Transform::identity(),  // frame A
    Transform::identity(),  // frame B
    true,  // use_linear_reference_frame_a
);

// 圆锥扭转约束
let constraint = world.create_cone_twist_constraint(
    handle_a,
    handle_b,
    Transform::identity(),
    Transform::identity(),
);
```

### 移除约束

```rust
world.remove_constraint(constraint_handle);
```

### 统计

```rust
let num_constraints = world.get_num_constraints();
```

## Ghost 对象管理

```rust
// 添加 Ghost 对象
let ghost_handle = world.add_ghost(ghost);

// 获取 Ghost 对象
if let Some(ghost) = world.get_ghost(ghost_handle) {
    let count = ghost.get_num_overlapping_objects();
}

// 移除 Ghost 对象
world.remove_ghost(ghost_handle);
```

## 射线检测

### 单次检测

```rust
let from = Vec3::new(0.0, 10.0, 0.0);
let to = Vec3::new(0.0, -10.0, 0.0);

if let Some(hit) = world.ray_test_closest(from, to) {
    println!("Hit point: {:?}", hit.hit_point);
    println!("Hit normal: {:?}", hit.hit_normal);
    println!("Hit fraction: {}", hit.hit_fraction);
    
    // 获取碰撞的刚体句柄
    if let Some(body_handle) = hit.get_rigid_body_handle(&world) {
        println!("Hit body: {}", body_handle);
    }
}
```

### 多次检测

```rust
let hits = world.ray_test_all(from, to, 10);  // 最多 10 个结果

for hit in hits {
    println!("Hit at: {:?}", hit.hit_point);
}
```

## 回调

### 接触回调

```rust
world.set_contact_callback(|contact: &ContactPoint| {
    println!("Contact at: {:?}", contact.position);
    println!("Normal: {:?}", contact.normal);
    println!("Distance: {}", contact.distance);
});

// 清除回调
world.clear_contact_callback();
```

### 碰撞过滤

```rust
world.set_collision_filter(|body_a: RigidBodyHandle, body_b: RigidBodyHandle| -> bool {
    // 返回 true 允许碰撞，返回 false 忽略碰撞
    true
});

// 清除过滤器
world.clear_collision_filter();
```

## 软体管理

```rust
// 添加软体
world.add_softbody(&softbody);

// 移除软体
world.remove_softbody(&softbody);

// 统计
let num_softbodies = world.num_softbodies();
```

## 车辆管理

```rust
// 添加车辆
world.add_vehicle(&vehicle);

// 移除车辆
world.remove_vehicle(&vehicle);
```

## 角色控制器管理

```rust
// 添加角色控制器
world.add_character(&character);

// 移除角色控制器
world.remove_character(&character);
```

## BroadphaseType

宽相检测算法类型。

```rust
pub enum BroadphaseType {
    DynamicAabbTree,  // 动态 AABB 树（默认）
    AxisSweep3,       // 轴扫描
}
```

## RayTestResult

射线检测结果。

```rust
pub struct RayTestResult {
    pub hit_point: Vec3,       // 碰撞点
    pub hit_normal: Vec3,      // 碰撞法线
    pub hit_fraction: Real,    // 碰撞比例 (0-1)
}

impl RayTestResult {
    pub fn get_rigid_body_handle(&self, world: &PhysicsWorld) -> Option<RigidBodyHandle>;
}
```

## ContactPoint

接触点信息。

```rust
pub struct ContactPoint {
    pub position: Vec3,   // 接触位置
    pub normal: Vec3,     // 接触法线
    pub distance: Real,   // 穿透深度
}

impl ContactPoint {
    pub fn get_body_a_handle(&self, world: &PhysicsWorld) -> Option<RigidBodyHandle>;
    pub fn get_body_b_handle(&self, world: &PhysicsWorld) -> Option<RigidBodyHandle>;
}
```

## 完整示例

```rust
use nekobullet::*;

fn main() {
    // 创建物理世界
    let mut world = PhysicsWorldBuilder::new()
        .gravity(Vec3::new(0.0, -9.81, 0.0))
        .build();

    // 创建地面
    let ground_shape = CollisionShapeBuilder::new()
        .box_shape(Vec3::new(50.0, 0.5, 50.0))
        .build()
        .unwrap();
    
    let ground = RigidBodyBuilder::new()
        .shape(ground_shape)
        .mass(0.0)
        .build()
        .unwrap();
    
    world.add_rigid_body(ground);

    // 创建多个球
    let ball_shape = CollisionShapeBuilder::new()
        .sphere(0.5)
        .build()
        .unwrap();

    let mut ball_handles = Vec::new();
    for i in 0..10 {
        let ball = RigidBodyBuilder::new()
            .shape(ball_shape.clone())
            .mass(1.0)
            .position(Vec3::new(i as f32 - 5.0, 10.0, 0.0))
            .build()
            .unwrap();
        
        ball_handles.push(world.add_rigid_body(ball));
    }

    // 设置接触回调
    world.set_contact_callback(|contact| {
        println!("Contact at {:?}", contact.position);
    });

    // 模拟循环
    for frame in 0..120 {
        world.step(1.0 / 60.0);
        
        println!("Frame {}", frame);
        for (i, &handle) in ball_handles.iter().enumerate() {
            if let Some(body) = world.get_rigid_body(handle) {
                let pos = body.get_position();
                println!("  Ball {}: y = {:.2}", i, pos.y);
            }
        }
    }
}
```
