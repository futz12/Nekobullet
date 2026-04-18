# Nekobullet

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Nekobullet 是 [Bullet Physics](https://pybullet.org/) 物理引擎的 Rust 绑定库，提供高性能的 3D 物理模拟功能。

## 特性

- **完整的物理模拟**：支持刚体、软体、约束、车辆、角色控制器等
- **高性能**：基于 Bullet Physics 的高效 C++ 实现
- **安全封装**：提供安全的 Rust API，同时保持底层性能
- **跨平台**：支持原生平台和 WebAssembly
- **100% API 覆盖**：完整绑定 Bullet Physics 核心功能

## 模块

### 核心模块

| 模块 | 描述 |
|------|------|
| PhysicsWorld | 物理世界，管理所有物理对象和模拟 |
| RigidBody | 刚体，用于模拟固体物体 |
| CollisionShape | 碰撞形状，定义物体的碰撞边界 |
| Constraint | 约束，限制物体之间的相对运动 |

### 高级功能

| 模块 | 描述 |
|------|------|
| SoftBody | 软体，模拟可变形物体 |
| Vehicle | 车辆，模拟轮式车辆 |
| CharacterController | 角色控制器，用于游戏角色移动 |
| GhostObject | Ghost 对象，用于碰撞检测而不产生物理响应 |
| GImpact | GImpact 形状，用于动态三角形网格碰撞 |
| HACD/VHACD | 凸分解，将凹网格分解为凸部件 |
| InverseDynamics | 逆动力学，计算关节力矩 |
| MLCPSolver | MLCP 求解器，高级数值求解 |
| ReducedDeformableBody | 简化软体，基于模态的软体模拟 |
| MultiBodyConstraint | 多体约束，Featherstone 算法约束 |

## 快速开始

### 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
nekobullet = "0.1"
```

### 基本示例

```rust
use nekobullet::*;

fn main() {
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
}
```

## 文档

完整文档位于 `rust/docs/` 目录：

- [快速入门指南](rust/docs/quickstart.md)
- [类型系统](rust/docs/types.md)
- [核心模块](rust/docs/core/README.md)
- [高级功能](rust/docs/advanced/README.md)
- [API 参考](rust/docs/api/README.md)

## 构建

### 原生平台

```bash
cd Nekobullet/rust
cargo build --release
```

### WebAssembly

```bash
cd Nekobullet/rust
cargo build --target wasm32-unknown-unknown --release
```

## 项目结构

```
Nekobullet/
├── bullet3/              # Bullet Physics 源码
├── cpp_bind/             # C++ 绑定层
├── rust/                 # Rust 绑定
│   ├── src/
│   │   ├── ffi/          # FFI 声明
│   │   └── core/         # Rust 安全封装
│   └── docs/             # 文档
└── Extras/               # Bullet 扩展模块
    ├── ConvexDecomposition/
    ├── GIMPACTUtils/
    ├── HACD/
    ├── InverseDynamics/
    └── VHACD/
```

## 支持的碰撞形状

- **基本形状**：Box、Sphere、Capsule、Cylinder、Cone、Plane
- **复杂形状**：ConvexHull、Compound、TriangleMesh、Heightfield
- **动态形状**：GImpact（动态凹网格）

## 支持的约束类型

- Point2Point（点对点）
- Hinge（铰链）
- Slider（滑动）
- Generic6Dof（6自由度）
- Generic6DofSpring（带弹簧的6自由度）
- ConeTwist（圆锥扭转）
- Universal（万向节）
- Hinge2（双铰链）
- Gear（齿轮）

## 许可证

本项目采用 MIT 许可证。Bullet Physics 采用 zlib 许可证。

## 致谢

- [Bullet Physics](https://github.com/bulletphysics/bullet3) - 物理引擎
- [glam](https://github.com/bitshifter/glam-rs) - 数学库
