# 快速入门指南

本指南将帮助你快速上手 Nekobullet 物理引擎。

## 基本概念

### 物理世界 (PhysicsWorld)

物理世界是所有物理对象的容器，负责管理物理模拟。

```rust
use nekobullet::*;

// 创建默认物理世界
let world = PhysicsWorld::new();

// 使用构建器创建自定义世界
let world = PhysicsWorldBuilder::new()
    .gravity(Vec3::new(0.0, -10.0, 0.0))
    .broadphase(BroadphaseType::DynamicAabbTree)
    .solver_iterations(10)
    .build();
```

### 碰撞形状 (CollisionShape)

碰撞形状定义了物体的碰撞边界。

```rust
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

// 凸包形状
let points = vec![
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 0.0, 1.0),
];
let convex_hull = CollisionShapeBuilder::new()
    .convex_hull(points)
    .build()
    .unwrap();

// 复合形状
let mut compound = CollisionShapeBuilder::new()
    .build()
    .unwrap();
// 添加子形状...
```

### 刚体 (RigidBody)

刚体是具有质量和碰撞形状的物理对象。

```rust
// 创建动态刚体
let body = RigidBodyBuilder::new()
    .shape(sphere_shape)
    .mass(1.0)
    .position(Vec3::new(0.0, 10.0, 0.0))
    .friction(0.5)
    .restitution(0.3)
    .build()
    .unwrap();

// 创建静态刚体（质量为 0）
let ground = RigidBodyBuilder::new()
    .shape(plane_shape)
    .mass(0.0)
    .build()
    .unwrap();

// 创建运动学刚体
let kinematic = RigidBodyBuilder::new()
    .shape(box_shape)
    .kinematic()
    .build()
    .unwrap();
```

## 完整示例

### 基本物理模拟

```rust
use nekobullet::*;

fn main() {
    // 创建物理世界
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 创建地面
    let ground_shape = CollisionShapeBuilder::new()
        .box_shape(Vec3::new(50.0, 0.5, 50.0))
        .build()
        .unwrap();
    
    let ground = RigidBodyBuilder::new()
        .shape(ground_shape)
        .mass(0.0)  // 静态物体
        .position(Vec3::new(0.0, -0.5, 0.0))
        .build()
        .unwrap();
    
    world.add_rigid_body(ground);

    // 创建下落的球
    let ball_shape = CollisionShapeBuilder::new()
        .sphere(0.5)
        .build()
        .unwrap();
    
    let ball = RigidBodyBuilder::new()
        .shape(ball_shape)
        .mass(1.0)
        .position(Vec3::new(0.0, 10.0, 0.0))
        .restitution(0.7)  // 弹性
        .build()
        .unwrap();
    
    let ball_handle = world.add_rigid_body(ball);

    // 模拟循环
    for i in 0..120 {
        world.step(1.0 / 60.0);
        
        let pos = world.get_rigid_body(ball_handle).unwrap().get_position();
        println!("Frame {}: Ball at ({:.2}, {:.2}, {:.2})", 
                 i, pos.x, pos.y, pos.z);
    }
}
```

### 约束示例

```rust
use nekobullet::*;

fn main() {
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 创建两个球
    let shape = CollisionShapeBuilder::new()
        .sphere(0.5)
        .build()
        .unwrap();

    let body_a = RigidBodyBuilder::new()
        .shape(shape.clone())
        .mass(1.0)
        .position(Vec3::new(-1.0, 5.0, 0.0))
        .build()
        .unwrap();

    let body_b = RigidBodyBuilder::new()
        .shape(shape)
        .mass(1.0)
        .position(Vec3::new(1.0, 5.0, 0.0))
        .build()
        .unwrap();

    let handle_a = world.add_rigid_body(body_a);
    let handle_b = world.add_rigid_body(body_b);

    // 创建点对点约束
    let constraint = world.create_point2point_constraint(
        handle_a,
        handle_b,
        Vec3::new(0.5, 0.0, 0.0),   // body A 的锚点
        Vec3::new(-0.5, 0.0, 0.0),  // body B 的锚点
    );

    // 模拟
    for _ in 0..120 {
        world.step(1.0 / 60.0);
    }
}
```

### 射线检测

```rust
use nekobullet::*;

fn main() {
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 创建一个球
    let shape = CollisionShapeBuilder::new()
        .sphere(1.0)
        .build()
        .unwrap();

    let body = RigidBodyBuilder::new()
        .shape(shape)
        .mass(1.0)
        .position(Vec3::new(0.0, 0.0, 0.0))
        .build()
        .unwrap();

    world.add_rigid_body(body);

    // 射线检测
    let from = Vec3::new(0.0, 10.0, 0.0);
    let to = Vec3::new(0.0, -10.0, 0.0);

    if let Some(hit) = world.ray_test_closest(from, to) {
        println!("Hit at: {:?}", hit.hit_point);
        println!("Normal: {:?}", hit.hit_normal);
        println!("Fraction: {}", hit.hit_fraction);
    }
}
```

## 下一步

- 阅读 [类型系统](./types.md) 了解基础数据类型
- 阅读 [核心模块](./core/README.md) 了解详细 API
- 阅读 [高级功能](./advanced/README.md) 了解软体、车辆等高级特性
