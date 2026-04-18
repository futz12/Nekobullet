# 类型系统

Nekobullet 使用 `glam` 库作为基础数学类型，并定义了一些特定的物理类型。

## 基础类型

### Real

```rust
pub type Real = f32;
```

浮点数类型别名，用于所有物理计算。

### Vec3

三维向量，来自 `glam` 库。

```rust
use nekobullet::Vec3;

// 创建向量
let v = Vec3::new(1.0, 2.0, 3.0);

// 常量
let zero = Vec3::ZERO;      // (0, 0, 0)
let one = Vec3::ONE;        // (1, 1, 1)
let x = Vec3::X;            // (1, 0, 0)
let y = Vec3::Y;            // (0, 1, 0)
let z = Vec3::Z;            // (0, 0, 1)

// 运算
let a = Vec3::new(1.0, 2.0, 3.0);
let b = Vec3::new(4.0, 5.0, 6.0);

let sum = a + b;            // 加法
let diff = a - b;           // 减法
let scaled = a * 2.0;       // 标量乘法
let dot = a.dot(b);         // 点积
let cross = a.cross(b);     // 叉积
let length = a.length();    // 长度
let normalized = a.normalize(); // 归一化
let lerp = a.lerp(b, 0.5);  // 线性插值
```

### Quat

四元数，用于表示旋转，来自 `glam` 库。

```rust
use nekobullet::Quat;

// 创建四元数
let identity = Quat::IDENTITY;  // 单位四元数

// 从欧拉角创建
let rot_x = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4);
let rot_y = Quat::from_rotation_y(std::f32::consts::FRAC_PI_4);
let rot_z = Quat::from_rotation_z(std::f32::consts::FRAC_PI_4);

// 从轴角创建
let axis = Vec3::new(0.0, 1.0, 0.0);
let angle = std::f32::consts::FRAC_PI_2;
let rot = Quat::from_axis_angle(axis, angle);

// 运算
let combined = rot_x * rot_y;   // 组合旋转
let rotated = rot * Vec3::X;    // 旋转向量
let inverse = rot.inverse();    // 逆旋转
let normalized = rot.normalize(); // 归一化
```

### Mat4

4x4 矩阵，来自 `glam` 库。

```rust
use nekobullet::Mat4;

// 创建矩阵
let identity = Mat4::IDENTITY;

// 从变换创建
let translation = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
let rotation = Mat4::from_quat(Quat::from_rotation_y(std::f32::consts::PI));
let scale = Mat4::from_scale(Vec3::new(2.0, 2.0, 2.0));

// 运算
let combined = translation * rotation;
let transformed = matrix * Vec4::new(1.0, 0.0, 0.0, 1.0);
```

## 物理类型

### Transform

变换结构，包含位置和旋转。

```rust
use nekobullet::Transform;

// 创建变换
let t = Transform::new(
    Vec3::new(1.0, 2.0, 3.0),  // 位置
    Quat::from_rotation_y(std::f32::consts::PI), // 旋转
);

// 从位置创建（无旋转）
let t = Transform::from_position(Vec3::new(0.0, 5.0, 0.0));

// 单位变换
let identity = Transform::identity();

// 转换为矩阵
let mat = t.to_mat4();

// 从矩阵创建
let t2 = Transform::from_mat4(&mat);
```

### Aabb

轴对齐包围盒。

```rust
use nekobullet::Aabb;

// 创建 AABB
let aabb = Aabb::new(
    Vec3::new(-1.0, -1.0, -1.0),  // 最小点
    Vec3::new(1.0, 1.0, 1.0),     // 最大点
);

// 属性
let center = aabb.center();    // 中心点
let extents = aabb.extents();  // 半尺寸
let size = aabb.size();        // 全尺寸
```

## 枚举类型

### CollisionShapeType

碰撞形状类型。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CollisionShapeType {
    Box = 0,
    Sphere = 1,
    Capsule = 2,
    Cylinder = 3,
    Cone = 4,
    Compound = 5,
    ConvexHull = 6,
    TriangleMesh = 7,
    Heightfield = 8,
    StaticPlane = 9,
}
```

### MotionType

刚体运动类型。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum MotionType {
    Static = 0,      // 静态物体，不移动
    Kinematic = 1,   // 运动学物体，由用户控制
    #[default]
    Dynamic = 2,     // 动态物体，受物理模拟影响
}
```

使用示例：

```rust
// 静态物体（如地面）
let ground = RigidBodyBuilder::new()
    .shape(shape)
    .mass(0.0)  // 质量为 0 自动设为静态
    .build()
    .unwrap();

// 运动学物体（如移动平台）
let platform = RigidBodyBuilder::new()
    .shape(shape)
    .kinematic()
    .build()
    .unwrap();

// 动态物体（如球）
let ball = RigidBodyBuilder::new()
    .shape(shape)
    .mass(1.0)  // 有质量自动设为动态
    .build()
    .unwrap();
```

### ActivationState

刚体激活状态。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum ActivationState {
    Inactive = 0,           // 非激活（睡眠）
    #[default]
    Active = 1,             // 激活
    DisableDeactivation = 2, // 禁用睡眠
    DisableSimulation = 3,  // 禁用模拟
}
```

使用示例：

```rust
// 检查激活状态
if body.is_active() {
    println!("Body is active");
}

// 激活物体
body.activate();

// 禁用睡眠（物体始终活跃）
body.set_activation_state(ActivationState::DisableDeactivation);
```

### ConstraintType

约束类型。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ConstraintType {
    Point2Point = 0,
    Hinge = 1,
    Slider = 2,
    Fixed = 3,
    Generic6Dof = 4,
    ConeTwist = 5,
    Generic6DofSpring = 6,
    Universal = 7,
    Hinge2 = 8,
    Gear = 9,
    Generic6DofSpring2 = 10,
    Unknown = 255,
}
```

### JointType

关节类型（用于逆动力学）。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum JointType {
    Fixed = 0,      // 固定关节
    Revolute = 1,   // 旋转关节
    Prismatic = 2,  // 滑动关节
    Floating = 3,   // 浮动关节
    Spherical = 4,  // 球关节
}
```

### RotateOrder

旋转顺序（用于 Generic6DofSpring2Constraint）。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RotateOrder {
    XYZ = 0,
    XZY = 1,
    YXZ = 2,
    YZX = 3,
    ZXY = 4,
    ZYX = 5,
}
```

## 句柄类型

### RigidBodyHandle

```rust
pub type RigidBodyHandle = u64;
```

刚体句柄，用于在物理世界中引用刚体。

### ConstraintHandle

```rust
pub type ConstraintHandle = u64;
```

约束句柄，用于在物理世界中引用约束。

### GhostHandle

```rust
pub type GhostHandle = u64;
```

Ghost 对象句柄，用于在物理世界中引用 Ghost 对象。
