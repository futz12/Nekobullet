# 核心模块

核心模块提供 Nekobullet 的基础物理功能。

## 模块列表

| 模块 | 描述 |
|------|------|
| [PhysicsWorld](./world.md) | 物理世界，管理所有物理对象和模拟 |
| [RigidBody](./rigidbody.md) | 刚体，用于模拟固体物体 |
| [CollisionShape](./collision.md) | 碰撞形状，定义物体的碰撞边界 |
| [Constraint](./constraint.md) | 约束，限制物体之间的相对运动 |

## 快速开始

```rust
use nekobullet::*;

// 1. 创建物理世界
let mut world = PhysicsWorld::new();
world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

// 2. 创建碰撞形状
let shape = CollisionShapeBuilder::new()
    .sphere(0.5)
    .build()
    .unwrap();

// 3. 创建刚体
let body = RigidBodyBuilder::new()
    .shape(shape)
    .mass(1.0)
    .position(Vec3::new(0.0, 10.0, 0.0))
    .build()
    .unwrap();

// 4. 添加到世界
let handle = world.add_rigid_body(body);

// 5. 模拟
for _ in 0..60 {
    world.step(1.0 / 60.0);
}

// 6. 获取结果
let pos = world.get_rigid_body(handle).unwrap().get_position();
println!("Position: {:?}", pos);
```

## 典型工作流程

1. **创建物理世界** - 设置重力、宽相等参数
2. **创建碰撞形状** - 定义物体的碰撞边界
3. **创建刚体** - 组合形状、质量、初始变换等
4. **添加到世界** - 将刚体添加到物理世界
5. **模拟** - 每帧调用 `step()` 推进模拟
6. **获取结果** - 读取位置、速度等状态

## 性能提示

- 使用适当的碰撞形状复杂度
- 合理设置睡眠阈值
- 使用碰撞过滤减少不必要的碰撞检测
- 对于静态物体，使用 `mass(0.0)` 创建
