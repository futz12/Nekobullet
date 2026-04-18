# Nekobullet 文档

Nekobullet 是 [Bullet Physics](https://pybullet.org/) 物理引擎的 Rust 绑定库，提供高性能的 3D 物理模拟功能。

## 特性

- **完整的物理模拟**：支持刚体、软体、约束、车辆、角色控制器等
- **高性能**：基于 Bullet Physics 的高效实现
- **安全封装**：提供安全的 Rust API，同时保持底层性能
- **跨平台**：支持原生平台和 WebAssembly

## 模块概览

### 核心模块

| 模块 | 描述 |
|------|------|
| [PhysicsWorld](./core/world.md) | 物理世界，管理所有物理对象和模拟 |
| [RigidBody](./core/rigidbody.md) | 刚体，用于模拟固体物体 |
| [CollisionShape](./core/collision.md) | 碰撞形状，定义物体的碰撞边界 |
| [Constraint](./core/constraint.md) | 约束，限制物体之间的相对运动 |

### 高级功能

| 模块 | 描述 |
|------|------|
| [SoftBody](./advanced/softbody.md) | 软体，模拟可变形物体 |
| [Vehicle](./advanced/vehicle.md) | 车辆，模拟轮式车辆 |
| [Character](./advanced/character.md) | 角色控制器，用于游戏角色移动 |
| [GhostObject](./advanced/ghost.md) | Ghost 对象，用于碰撞检测而不产生物理响应 |
| [GImpact](./advanced/gimpact.md) | GImpact 形状，用于动态三角形网格碰撞 |
| [HACD/VHACD](./advanced/hacd_vhacd.md) | 凸分解，将凹网格分解为凸部件 |
| [InverseDynamics](./advanced/inverse_dynamics.md) | 逆动力学，计算关节力矩 |
| [MLCPSolver](./advanced/mlcp_solver.md) | MLCP 求解器，高级数值求解 |
| [ReducedDeformableBody](./advanced/reduced_softbody.md) | 简化软体，基于模态的软体模拟 |
| [MultiBodyConstraint](./advanced/multibody_constraint.md) | 多体约束，Featherstone 算法约束 |

## 快速开始

```rust
use nekobullet::*;

// 创建物理世界
let mut world = PhysicsWorld::new();
world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

// 创建碰撞形状
let shape = CollisionShapeBuilder::new()
    .box_shape(Vec3::new(1.0, 1.0, 1.0))
    .build()
    .unwrap();

// 创建刚体
let body = RigidBodyBuilder::new()
    .shape(shape)
    .mass(1.0)
    .position(Vec3::new(0.0, 10.0, 0.0))
    .build()
    .unwrap();

// 添加到世界
let handle = world.add_rigid_body(body);

// 模拟
for _ in 0..60 {
    world.step(1.0 / 60.0);
}

// 获取位置
let pos = world.get_rigid_body(handle).unwrap().get_position();
println!("Position: {:?}", pos);
```

## 文档导航

- [快速入门指南](./quickstart.md)
- [类型系统](./types.md)
- [核心模块](./core/README.md)
- [高级功能](./advanced/README.md)
- [API 参考](./api/README.md)

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
nekobullet = "0.1"
```

## 许可证

本项目采用 MIT 许可证。
