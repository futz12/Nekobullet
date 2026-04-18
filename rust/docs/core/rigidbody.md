# RigidBody

刚体是具有质量和碰撞形状的物理对象，是物理模拟的核心元素。

## 创建

### 使用 RigidBodyBuilder

```rust
use nekobullet::*;

let body = RigidBodyBuilder::new()
    .shape(shape)                    // 必需：碰撞形状
    .mass(1.0)                       // 质量
    .position(Vec3::new(0.0, 5.0, 0.0))  // 初始位置
    .rotation(Quat::from_rotation_y(PI)) // 初始旋转
    .linear_velocity(Vec3::new(1.0, 0.0, 0.0))  // 初始线速度
    .angular_velocity(Vec3::new(0.0, 1.0, 0.0)) // 初始角速度
    .friction(0.5)                   // 摩擦系数
    .restitution(0.3)                // 弹性系数
    .damping(0.1, 0.1)               // 线性/角阻尼
    .build()
    .unwrap();
```

### 刚体类型

```rust
// 动态物体（受物理模拟影响）
let dynamic = RigidBodyBuilder::new()
    .shape(shape.clone())
    .mass(1.0)
    .dynamic()
    .build()
    .unwrap();

// 静态物体（不移动，如地面）
let static_body = RigidBodyBuilder::new()
    .shape(shape.clone())
    .mass(0.0)  // 质量为 0 自动设为静态
    .static_body()
    .build()
    .unwrap();

// 运动学物体（由用户控制，不受物理影响）
let kinematic = RigidBodyBuilder::new()
    .shape(shape)
    .kinematic()
    .build()
    .unwrap();
```

### 碰撞过滤

```rust
let body = RigidBodyBuilder::new()
    .shape(shape)
    .mass(1.0)
    .collision_filter(1, 0xFFFF)  // 组, 掩码
    .build()
    .unwrap();
```

### 睡眠设置

```rust
let body = RigidBodyBuilder::new()
    .shape(shape)
    .mass(1.0)
    .sleeping_threshold(0.8, 1.0)  // 线性, 角度阈值
    .disable_deactivation(true)    // 禁用睡眠
    .build()
    .unwrap();
```

## 变换

### 位置

```rust
// 获取位置
let pos = body.get_position();

// 设置位置
body.set_position(Vec3::new(0.0, 5.0, 0.0));
```

### 旋转

```rust
// 获取旋转
let rot = body.get_rotation();

// 设置旋转
body.set_rotation(Quat::from_rotation_y(std::f32::consts::PI));
```

### 变换

```rust
// 获取完整变换
let transform = body.get_transform();

// 设置完整变换
body.set_transform(&Transform::new(
    Vec3::new(0.0, 5.0, 0.0),
    Quat::from_rotation_x(std::f32::consts::FRAC_PI_4),
));
```

## 速度

### 线速度

```rust
// 获取线速度
let vel = body.get_linear_velocity();

// 设置线速度
body.set_linear_velocity(Vec3::new(1.0, 0.0, 0.0));
```

### 角速度

```rust
// 获取角速度
let vel = body.get_angular_velocity();

// 设置角速度
body.set_angular_velocity(Vec3::new(0.0, 1.0, 0.0));
```

## 力和冲量

### 力

```rust
// 在质心施加力
body.apply_central_force(Vec3::new(0.0, 100.0, 0.0));

// 在指定点施加力
body.apply_force(Vec3::new(0.0, 100.0, 0.0));

// 施加扭矩
body.apply_torque(Vec3::new(0.0, 10.0, 0.0));

// 清除所有力
body.clear_forces();
```

### 冲量

```rust
// 在质心施加冲量
body.apply_central_impulse(Vec3::new(0.0, 10.0, 0.0));

// 在指定点施加冲量
body.apply_impulse(Vec3::new(0.0, 10.0, 0.0));

// 施加扭矩冲量
body.apply_torque_impulse(Vec3::new(0.0, 1.0, 0.0));
```

## 质量属性

```rust
// 获取质量
let mass = body.get_mass();

// 设置质量
body.set_mass(2.0);

// 获取逆质量
let inv_mass = body.get_inverse_mass();

// 计算惯性张量
let inertia = shape.calculate_local_inertia(mass);
```

## 摩擦和弹性

### 摩擦

```rust
// 获取摩擦系数
let friction = body.get_friction();

// 设置摩擦系数
body.set_friction(0.5);

// 滚动摩擦
let rolling = body.get_rolling_friction();
body.set_rolling_friction(0.1);

// 旋转摩擦
let spinning = body.get_spinning_friction();
body.set_spinning_friction(0.1);
```

### 弹性

```rust
// 获取弹性系数
let restitution = body.get_restitution();

// 设置弹性系数
body.set_restitution(0.8);
```

## 阻尼

```rust
// 获取阻尼
let linear_damping = body.get_linear_damping();
let angular_damping = body.get_angular_damping();

// 设置阻尼
body.set_damping(0.1, 0.1);  // 线性, 角度
```

## 睡眠

```rust
// 获取激活状态
let state = body.get_activation_state();

// 设置激活状态
body.set_activation_state(ActivationState::Active);

// 激活物体
body.activate();

// 检查是否激活
if body.is_active() {
    println!("Body is active");
}

// 设置睡眠阈值
body.set_sleeping_thresholds(0.8, 1.0);

// 获取睡眠阈值
let linear_threshold = body.get_linear_sleeping_threshold();
let angular_threshold = body.get_angular_sleeping_threshold();
```

## 类型检查

```rust
// 检查刚体类型
if body.is_static() {
    println!("Static body");
}

if body.is_kinematic() {
    println!("Kinematic body");
}

if body.is_dynamic() {
    println!("Dynamic body");
}

// 设置类型
body.set_static(true);
body.set_kinematic(true);
```

## 碰撞形状

```rust
// 获取碰撞形状
if let Some(shape) = body.get_collision_shape() {
    let shape_type = shape.shape_type();
}

// 设置碰撞形状
body.set_collision_shape(new_shape);
```

## AABB

```rust
// 获取轴对齐包围盒
let aabb = body.get_aabb();
println!("AABB min: {:?}", aabb.min);
println!("AABB max: {:?}", aabb.max);
println!("Center: {:?}", aabb.center());
println!("Size: {:?}", aabb.size());
```

## 碰撞过滤

```rust
// 设置碰撞组
body.set_collision_group(1);

// 获取碰撞组
let group = body.get_collision_group();

// 设置碰撞掩码
body.set_collision_mask(0xFFFF);

// 获取碰撞掩码
let mask = body.get_collision_mask();
```

## RigidBodyBuilder 完整 API

```rust
impl RigidBodyBuilder {
    pub fn new() -> Self;
    
    // 形状和质量
    pub fn shape(self, shape: ShapeHandle) -> Self;
    pub fn mass(self, mass: Real) -> Self;
    
    // 变换
    pub fn position(self, position: Vec3) -> Self;
    pub fn rotation(self, rotation: Quat) -> Self;
    pub fn transform(self, transform: Transform) -> Self;
    
    // 速度
    pub fn linear_velocity(self, velocity: Vec3) -> Self;
    pub fn angular_velocity(self, velocity: Vec3) -> Self;
    
    // 物理属性
    pub fn friction(self, friction: Real) -> Self;
    pub fn restitution(self, restitution: Real) -> Self;
    pub fn damping(self, linear: Real, angular: Real) -> Self;
    
    // 类型
    pub fn motion_type(self, motion_type: MotionType) -> Self;
    pub fn static_body(self) -> Self;
    pub fn kinematic(self) -> Self;
    pub fn dynamic(self) -> Self;
    
    // 碰撞
    pub fn collision_filter(self, group: u16, mask: u16) -> Self;
    
    // 睡眠
    pub fn sleeping_threshold(self, linear: Real, angular: Real) -> Self;
    pub fn disable_deactivation(self, disable: bool) -> Self;
    
    // 构建
    pub fn build(self) -> Result<RigidBody, &'static str>;
}
```

## 完整示例

```rust
use nekobullet::*;

fn main() {
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 创建地面
    let ground_shape = CollisionShapeBuilder::new()
        .box_shape(Vec3::new(10.0, 0.5, 10.0))
        .build()
        .unwrap();
    
    let ground = RigidBodyBuilder::new()
        .shape(ground_shape)
        .mass(0.0)
        .position(Vec3::new(0.0, -0.5, 0.0))
        .friction(0.8)
        .build()
        .unwrap();
    
    world.add_rigid_body(ground);

    // 创建弹跳球
    let ball_shape = CollisionShapeBuilder::new()
        .sphere(0.5)
        .build()
        .unwrap();
    
    let ball = RigidBodyBuilder::new()
        .shape(ball_shape)
        .mass(1.0)
        .position(Vec3::new(0.0, 10.0, 0.0))
        .linear_velocity(Vec3::new(1.0, 0.0, 0.0))
        .friction(0.3)
        .restitution(0.8)
        .damping(0.1, 0.1)
        .build()
        .unwrap();
    
    let ball_handle = world.add_rigid_body(ball);

    // 施加持续的力
    if let Some(body) = world.get_rigid_body(ball_handle) {
        body.apply_central_force(Vec3::new(0.0, 0.0, 5.0));
    }

    // 模拟
    for i in 0..120 {
        world.step(1.0 / 60.0);
        
        if let Some(body) = world.get_rigid_body(ball_handle) {
            let pos = body.get_position();
            let vel = body.get_linear_velocity();
            
            println!("Frame {}: pos=({:.2}, {:.2}, {:.2}), vel=({:.2}, {:.2}, {:.2})",
                     i, pos.x, pos.y, pos.z, vel.x, vel.y, vel.z);
        }
    }
}
```
