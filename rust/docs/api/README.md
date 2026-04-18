# API 参考索引

本文档提供 Nekobullet 所有公开 API 的快速索引。

## 核心模块

### PhysicsWorld

物理世界，管理所有物理对象和模拟。

| 方法 | 描述 |
|------|------|
| [`new()`](../core/world.md#创建) | 创建默认物理世界 |
| [`set_gravity(gravity: Vec3)`](../core/world.md#属性设置) | 设置重力 |
| [`step(dt: Real)`](../core/world.md#物理模拟) | 步进模拟 |
| [`add_rigid_body(body: RigidBody)`](../core/world.md#刚体管理) | 添加刚体 |
| [`remove_rigid_body(handle: RigidBodyHandle)`](../core/world.md#刚体管理) | 移除刚体 |
| [`ray_test_closest(from: Vec3, to: Vec3)`](../core/world.md#射线检测) | 射线检测 |

[完整文档](../core/world.md)

### RigidBody

刚体，具有质量和碰撞形状的物理对象。

| 方法 | 描述 |
|------|------|
| [`get_position() -> Vec3`](../core/rigidbody.md#变换) | 获取位置 |
| [`set_position(pos: Vec3)`](../core/rigidbody.md#变换) | 设置位置 |
| [`get_linear_velocity() -> Vec3`](../core/rigidbody.md#速度) | 获取线速度 |
| [`apply_central_force(force: Vec3)`](../core/rigidbody.md#力和冲量) | 施加力 |
| [`set_mass(mass: Real)`](../core/rigidbody.md#质量属性) | 设置质量 |

[完整文档](../core/rigidbody.md)

### RigidBodyBuilder

刚体构建器。

| 方法 | 描述 |
|------|------|
| [`new()`](../core/rigidbody.md#创建) | 创建构建器 |
| [`shape(shape: ShapeHandle)`](../core/rigidbody.md#创建) | 设置形状 |
| [`mass(mass: Real)`](../core/rigidbody.md#创建) | 设置质量 |
| [`position(pos: Vec3)`](../core/rigidbody.md#创建) | 设置初始位置 |
| [`build() -> Result<RigidBody, &str>`](../core/rigidbody.md#创建) | 构建刚体 |

[完整文档](../core/rigidbody.md)

### CollisionShape / ShapeHandle

碰撞形状。

| 方法 | 描述 |
|------|------|
| [`new_box(half_extents: Vec3)`](../core/collision.md#创建形状) | 创建盒子形状 |
| [`new_sphere(radius: Real)`](../core/collision.md#创建形状) | 创建球体形状 |
| [`new_capsule(radius: Real, height: Real)`](../core/collision.md#创建形状) | 创建胶囊形状 |
| [`new_convex_hull(points: &[Vec3])`](../core/collision.md#凸包形状) | 创建凸包形状 |
| [`new_triangle_mesh(vertices: &[Vec3], indices: &[i32])`](../core/collision.md#三角形网格形状) | 创建三角形网格 |

[完整文档](../core/collision.md)

### CollisionShapeBuilder

碰撞形状构建器。

| 方法 | 描述 |
|------|------|
| [`new()`](../core/collision.md#创建形状) | 创建构建器 |
| [`box_shape(half_extents: Vec3)`](../core/collision.md#创建形状) | 设置盒子形状 |
| [`sphere(radius: Real)`](../core/collision.md#创建形状) | 设置球体形状 |
| [`build() -> Option<ShapeHandle>`](../core/collision.md#创建形状) | 构建形状 |

[完整文档](../core/collision.md)

### Constraint

约束。

| 类型 | 描述 |
|------|------|
| [`Point2Point`](../core/constraint.md#点对点约束-point2point) | 点对点约束 |
| [`Hinge`](../core/constraint.md#铰链约束-hinge) | 铰链约束 |
| [`Slider`](../core/constraint.md#滑动约束-slider) | 滑动约束 |
| [`Generic6Dof`](../core/constraint.md#6自由度约束-generic6dof) | 6自由度约束 |
| [`ConeTwist`](../core/constraint.md#圆锥扭转约束-conetwist) | 圆锥扭转约束 |

[完整文档](../core/constraint.md)

## 高级功能

### SoftBody

软体。

| 方法 | 描述 |
|------|------|
| [`create_rope(...)`](../advanced/softbody.md#创建绳索) | 创建绳索 |
| [`create_patch(...)`](../advanced/softbody.md#创建面片) | 创建面片 |
| [`create_ellipsoid(...)`](../advanced/softbody.md#创建椭球) | 创建椭球 |
| [`get_node_position(index: i32)`](../advanced/softbody.md#节点操作) | 获取节点位置 |
| [`set_total_mass(mass: Real)`](../advanced/softbody.md#质量操作) | 设置总质量 |

[完整文档](../advanced/softbody.md)

### Vehicle

车辆。

| 方法 | 描述 |
|------|------|
| [`new(chassis, raycaster, tuning)`](../advanced/vehicle.md#创建) | 创建车辆 |
| [`add_wheel(...)`](../advanced/vehicle.md#添加车轮) | 添加车轮 |
| [`set_steering(angle: Real, wheel: i32)`](../advanced/vehicle.md#控制) | 设置转向 |
| [`apply_engine_force(force: Real, wheel: i32)`](../advanced/vehicle.md#控制) | 施加引擎力 |
| [`set_brake(force: Real, wheel: i32)`](../advanced/vehicle.md#控制) | 设置刹车 |

[完整文档](../advanced/vehicle.md)

### CharacterController

角色控制器。

| 方法 | 描述 |
|------|------|
| [`new(ghost, shape, step_height, up)`](../advanced/character.md#创建) | 创建控制器 |
| [`set_walk_direction(dir: Vec3)`](../advanced/character.md#移动控制) | 设置行走方向 |
| [`jump(velocity: Vec3)`](../advanced/character.md#跳跃) | 跳跃 |
| [`on_ground() -> bool`](../advanced/character.md#跳跃) | 是否在地面 |

[完整文档](../advanced/character.md)

### GhostObject

Ghost 对象。

| 方法 | 描述 |
|------|------|
| [`new()`](../advanced/ghost.md#创建) | 创建 Ghost 对象 |
| [`set_shape(shape: ShapeHandle)`](../advanced/ghost.md#设置形状) | 设置形状 |
| [`set_position(pos: Vec3)`](../advanced/ghost.md#变换) | 设置位置 |
| [`get_num_overlapping_objects() -> i32`](../advanced/ghost.md#重叠检测) | 获取重叠对象数量 |

[完整文档](../advanced/ghost.md)

### GImpactShape

GImpact 形状。

| 方法 | 描述 |
|------|------|
| [`from_trimesh(trimesh, scale)`](../advanced/gimpact.md#创建) | 从三角形网格创建 |
| [`update_bound()`](../advanced/gimpact.md#属性) | 更新边界 |
| [`num_child_shapes() -> i32`](../advanced/gimpact.md#属性) | 获取子形状数量 |

[完整文档](../advanced/gimpact.md)

### HACD / VHACD

凸分解。

| 方法 | 描述 |
|------|------|
| [`new()`](../advanced/hacd_vhacd.md#创建和设置) | 创建分解器 |
| [`set_mesh(points, triangles)`](../advanced/hacd_vhacd.md#设置网格) | 设置网格 |
| [`compute()`](../advanced/hacd_vhacd.md#计算) | 执行分解 |
| [`get_num_hulls() -> i32`](../advanced/hacd_vhacd.md#获取结果) | 获取凸包数量 |
| [`get_hull(index: i32)`](../advanced/hacd_vhacd.md#获取结果) | 获取凸包 |

[完整文档](../advanced/hacd_vhacd.md)

### MultiBody

多体系统（逆动力学）。

| 方法 | 描述 |
|------|------|
| [`new()`](../advanced/inverse_dynamics.md#创建) | 创建多体系统 |
| [`add_body(...)`](../advanced/inverse_dynamics.md#添加物体) | 添加物体 |
| [`finalize()`](../advanced/inverse_dynamics.md#完成) | 完成构建 |
| [`calculate_inverse_dynamics(...)`](../advanced/inverse_dynamics.md#动力学计算) | 计算逆动力学 |

[完整文档](../advanced/inverse_dynamics.md)

### MLCPSolver

MLCP 求解器。

| 方法 | 描述 |
|------|------|
| [`new(interface: &DantzigSolver)`](../advanced/mlcp_solver.md#创建) | 创建求解器 |
| [`set_num_fallbacks(num: i32)`](../advanced/mlcp_solver.md#属性) | 设置回退次数 |

[完整文档](../advanced/mlcp_solver.md)

### ReducedDeformableBody

简化软体。

| 方法 | 描述 |
|------|------|
| [`new()`](../advanced/reduced_softbody.md#创建) | 创建简化软体 |
| [`set_num_modes(num: i32)`](../advanced/reduced_softbody.md#设置模态) | 设置模态数量 |
| [`get_rigid_transform() -> Transform`](../advanced/reduced_softbody.md#刚体变换) | 获取刚体变换 |

[完整文档](../advanced/reduced_softbody.md)

### MultiBodyConstraint

多体约束。

| 类型 | 描述 |
|------|------|
| [`MultiBodyPoint2PointConstraint`](../advanced/multibody_constraint.md#multibodypoint2pointconstraint) | 点对点约束 |
| [`MultiBodyFixedConstraint`](../advanced/multibody_constraint.md#multibodyfixedconstraint) | 固定约束 |
| [`MultiBodySliderConstraint`](../advanced/multibody_constraint.md#multibodysliderconstraint) | 滑动约束 |

[完整文档](../advanced/multibody_constraint.md)

## 类型

### 基础类型

| 类型 | 描述 |
|------|------|
| [`Real`](../types.md#基础类型) | 浮点数类型 (f32) |
| [`Vec3`](../types.md#vec3) | 三维向量 |
| [`Quat`](../types.md#quat) | 四元数 |
| [`Mat4`](../types.md#mat4) | 4x4 矩阵 |
| [`Transform`](../types.md#transform) | 变换 |
| [`Aabb`](../types.md#aabb) | 轴对齐包围盒 |

### 枚举

| 枚举 | 描述 |
|------|------|
| [`CollisionShapeType`](../types.md#collisionshapetype) | 碰撞形状类型 |
| [`MotionType`](../types.md#motiontype) | 运动类型 |
| [`ActivationState`](../types.md#activationstate) | 激活状态 |
| [`ConstraintType`](../types.md#constrainttype) | 约束类型 |
| [`JointType`](../types.md#jointtype) | 关节类型 |

[完整文档](../types.md)

### 句柄

| 句柄 | 描述 |
|------|------|
| [`RigidBodyHandle`](../types.md#句柄类型) | 刚体句柄 |
| [`ConstraintHandle`](../types.md#句柄类型) | 约束句柄 |
| [`GhostHandle`](../types.md#句柄类型) | Ghost 对象句柄 |

## 模块导航

- [核心模块](../core/README.md)
  - [PhysicsWorld](../core/world.md)
  - [RigidBody](../core/rigidbody.md)
  - [CollisionShape](../core/collision.md)
  - [Constraint](../core/constraint.md)
- [高级功能](../advanced/README.md)
  - [SoftBody](../advanced/softbody.md)
  - [Vehicle](../advanced/vehicle.md)
  - [Character](../advanced/character.md)
  - [GhostObject](../advanced/ghost.md)
  - [GImpact](../advanced/gimpact.md)
  - [HACD/VHACD](../advanced/hacd_vhacd.md)
  - [InverseDynamics](../advanced/inverse_dynamics.md)
  - [MLCPSolver](../advanced/mlcp_solver.md)
  - [ReducedDeformableBody](../advanced/reduced_softbody.md)
  - [MultiBodyConstraint](../advanced/multibody_constraint.md)
