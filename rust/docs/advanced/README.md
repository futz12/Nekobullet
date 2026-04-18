# 高级功能

高级功能模块提供 Nekobullet 的高级物理模拟功能。

## 模块列表

| 模块 | 描述 |
|------|------|
| [SoftBody](./softbody.md) | 软体，模拟可变形物体 |
| [Vehicle](./vehicle.md) | 车辆，模拟轮式车辆 |
| [Character](./character.md) | 角色控制器，用于游戏角色移动 |
| [GhostObject](./ghost.md) | Ghost 对象，用于碰撞检测而不产生物理响应 |
| [GImpact](./gimpact.md) | GImpact 形状，用于动态三角形网格碰撞 |
| [HACD/VHACD](./hacd_vhacd.md) | 凸分解，将凹网格分解为凸部件 |
| [InverseDynamics](./inverse_dynamics.md) | 逆动力学，计算关节力矩 |
| [MLCPSolver](./mlcp_solver.md) | MLCP 求解器，高级数值求解 |
| [ReducedDeformableBody](./reduced_softbody.md) | 简化软体，基于模态的软体模拟 |
| [MultiBodyConstraint](./multibody_constraint.md) | 多体约束，Featherstone 算法约束 |

## 功能概述

### 软体模拟

软体可以模拟布料、绳索、弹性物体等可变形物体。

```rust
// 创建软体世界信息
let world_info = SoftBodyWorldInfo::new();
world_info.set_gravity(Vec3::new(0.0, -9.81, 0.0));

// 创建绳索
let rope = SoftBody::create_rope(
    &world_info,
    Vec3::new(0.0, 10.0, 0.0),  // 起点
    Vec3::new(5.0, 10.0, 0.0),  // 终点
    10,  // 分辨率
    1,   // 固定点
);
```

### 车辆模拟

车辆系统提供完整的轮式车辆模拟。

```rust
// 创建车辆
let raycaster = VehicleRaycaster::new(&world);
let tuning = VehicleTuning::default();

let mut vehicle = Vehicle::new(&chassis, &raycaster, &tuning);

// 添加车轮
vehicle.add_wheel(
    Vec3::new(1.0, 0.0, 1.5),   // 连接点
    Vec3::new(0.0, -1.0, 0.0),  // 方向
    Vec3::new(-1.0, 0.0, 0.0),  // 轴
    0.3,  // 悬挂静止长度
    0.5,  // 车轮半径
    true, // 前轮
    &tuning,
);
```

### 角色控制器

角色控制器提供游戏角色的物理移动。

```rust
// 创建角色控制器
let ghost = GhostObject::new();
let shape = ShapeHandle::new_capsule(0.3, 1.0);

let character = CharacterController::new(
    &ghost,
    &shape,
    0.3,  // 步高
    Vec3::new(0.0, 1.0, 0.0),  // 上方向
);

// 移动角色
character.set_walk_direction(Vec3::new(0.0, 0.0, 1.0));
character.jump(Vec3::new(0.0, 5.0, 0.0));
```

### Ghost 对象

Ghost 对象用于碰撞检测而不产生物理响应。

```rust
let mut ghost = GhostObject::new();
ghost.set_shape(shape);
ghost.set_position(Vec3::new(0.0, 5.0, 0.0));

// 检测重叠对象
for obj in ghost.overlapping_objects() {
    println!("Overlapping with: {:?}", obj);
}
```

### 凸分解

HACD 和 VHACD 用于将凹网格分解为凸部件。

```rust
let mut vhacd = VHACD::new();
let params = VHACDParams::default();

if vhacd.compute(&points, &triangles, &params) {
    for i in 0..vhacd.get_num_hulls() {
        if let Some(hull) = vhacd.get_hull(i) {
            // 使用凸包...
        }
    }
}
```

### 逆动力学

逆动力学用于计算多体系统的关节力矩。

```rust
let mut multibody = MultiBody::new();
multibody.add_body(
    0,  // body_index
    -1, // parent_index
    JointType::Revolute,
    Vec3::ZERO,  // parent_r
    [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],  // body_t_parent
    Vec3::new(0.0, 1.0, 0.0),  // axis
    1.0,  // mass
    Vec3::ZERO,  // com
    [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],  // inertia
);
multibody.finalize();
```

## 使用场景

| 功能 | 使用场景 |
|------|----------|
| SoftBody | 布料、绳索、弹性物体、果冻 |
| Vehicle | 赛车游戏、车辆模拟 |
| Character | 第一/第三人称游戏角色 |
| GhostObject | 触发区域、传感器、AI 感知 |
| GImpact | 动态凹网格碰撞 |
| HACD/VHACD | 模型预处理、碰撞优化 |
| InverseDynamics | 机器人控制、动画物理 |
| MLCPSolver | 高精度物理模拟 |
| ReducedDeformableBody | 实时软体动画 |
| MultiBodyConstraint | 机器人、机械臂 |
