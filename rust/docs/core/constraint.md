# Constraint

约束用于限制刚体之间的相对运动，是构建复杂物理结构的关键。

## 约束类型

| 类型 | 描述 | 自由度 |
|------|------|--------|
| Point2Point | 点对点约束 | 3 (位置固定，可旋转) |
| Hinge | 铰链约束 | 1 (绕轴旋转) |
| Slider | 滑动约束 | 1 (沿轴滑动) |
| Generic6Dof | 6自由度约束 | 可配置 |
| Generic6DofSpring | 带弹簧的6自由度约束 | 可配置 |
| Generic6DofSpring2 | 改进的带弹簧6自由度约束 | 可配置 |
| ConeTwist | 圆锥扭转约束 | 锥形+扭转 |
| Universal | 万向节约束 | 2 旋转轴 |
| Hinge2 | 双铰链约束 | 2 旋转轴 |
| Gear | 齿轮约束 | 联动旋转 |

## 创建约束

### 使用 PhysicsWorld 创建

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
    Transform::identity(),
    Transform::identity(),
    true,
);

// 圆锥扭转约束
let constraint = world.create_cone_twist_constraint(
    handle_a,
    handle_b,
    Transform::identity(),
    Transform::identity(),
);
```

### 使用 ConstraintBuilder

```rust
// 点对点约束
let constraint_handle = ConstraintBuilder::new()
    .point2point(body_a.handle(), body_b.handle())
    .pivot_a(Vec3::new(0.5, 0.0, 0.0))
    .pivot_b(Vec3::new(-0.5, 0.0, 0.0))
    .build()
    .ok();

// 6自由度约束
let constraint_handle = ConstraintBuilder::new()
    .generic_6dof(body_a.handle(), body_b.handle())
    .frame_a(Transform::identity())
    .frame_b(Transform::identity())
    .linear_limits(
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(1.0, 1.0, 1.0),
    )
    .angular_limits(
        Vec3::new(-PI, -PI, -PI),
        Vec3::new(PI, PI, PI),
    )
    .build()
    .ok();
```

## 点对点约束 (Point2Point)

将两个刚体的指定点连接在一起。

```rust
let constraint = world.create_point2point_constraint(
    handle_a,
    handle_b,
    Vec3::new(0.5, 0.0, 0.0),   // body A 的锚点（局部坐标）
    Vec3::new(-0.5, 0.0, 0.0),  // body B 的锚点（局部坐标）
);
```

## 铰链约束 (Hinge)

允许两个刚体绕共同轴旋转。

```rust
let hinge = unsafe { HingeConstraint::new(
    body_a.handle(),
    body_b.handle(),
    Transform::from_position(Vec3::new(0.5, 0.0, 0.0)),  // frame A
    Transform::from_position(Vec3::new(-0.5, 0.0, 0.0)), // frame B
    -std::f32::consts::FRAC_PI_4,  // 下限
    std::f32::consts::FRAC_PI_4,   // 上限
) };

hinge.set_enabled(true);
```

## 滑动约束 (Slider)

允许两个刚体沿共同轴滑动。

```rust
let slider = unsafe { SliderConstraint::new(
    body_a.handle(),
    body_b.handle(),
    Transform::identity(),
    Transform::identity(),
) };

slider.set_enabled(true);
```

## 6自由度约束 (Generic6Dof)

最灵活的约束类型，可以独立限制每个自由度。

```rust
let constraint = unsafe { Generic6DofConstraint::new(
    body_a.handle(),
    body_b.handle(),
    Transform::identity(),
    Transform::identity(),
    true,  // use_linear_reference_frame_a
) };

// 设置线性限制
constraint.set_linear_lower_limit(Vec3::new(-1.0, 0.0, -1.0));
constraint.set_linear_upper_limit(Vec3::new(1.0, 0.0, 1.0));

// 设置角度限制
constraint.set_angular_lower_limit(Vec3::new(0.0, 0.0, 0.0));
constraint.set_angular_upper_limit(Vec3::new(0.0, 0.0, 0.0));

// 获取限制
let lower = constraint.get_linear_lower_limit();
let upper = constraint.get_linear_upper_limit();
```

## 带弹簧的6自由度约束 (Generic6DofSpring)

在 Generic6Dof 基础上添加弹簧效果。

```rust
let constraint = unsafe { Generic6DofSpringConstraint::new(
    body_a.handle(),
    body_b.handle(),
    Transform::identity(),
    Transform::identity(),
    true,
) };

// 启用弹簧（轴 0-5：X, Y, Z, RX, RY, RZ）
constraint.enable_spring(0, true);  // X 轴弹簧
constraint.enable_spring(1, true);  // Y 轴弹簧

// 设置弹簧参数
constraint.set_stiffness(0, 100.0);  // 刚度
constraint.set_damping(0, 0.5);      // 阻尼

// 设置平衡点
constraint.set_equilibrium_point(0, 0.0);

// 设置限制
constraint.set_linear_lower_limit(Vec3::new(-1.0, -1.0, -1.0));
constraint.set_linear_upper_limit(Vec3::new(1.0, 1.0, 1.0));
```

## 改进的6自由度弹簧约束 (Generic6DofSpring2)

改进版本，支持不同的旋转顺序。

```rust
let constraint = unsafe { Generic6DofSpring2Constraint::new(
    body_a.handle(),
    body_b.handle(),
    Transform::identity(),
    Transform::identity(),
    RotateOrder::XYZ,  // 旋转顺序
) };

constraint.set_linear_lower_limit(Vec3::new(-1.0, -1.0, -1.0));
constraint.set_linear_upper_limit(Vec3::new(1.0, 1.0, 1.0));
```

## 圆锥扭转约束 (ConeTwist)

允许在圆锥范围内摆动和扭转。

```rust
let constraint = unsafe { ConeTwistConstraint::new(
    body_a.handle(),
    body_b.handle(),
    Transform::identity(),
    Transform::identity(),
) };

// 设置限制
constraint.set_limit(
    std::f32::consts::FRAC_PI_4,  // swing_span1
    std::f32::consts::FRAC_PI_4,  // swing_span2
    std::f32::consts::FRAC_PI_2,  // twist_span
    1.0,  // softness
    0.3,  // bias_factor
    0.9,  // relaxation_factor
);

// 设置电机目标
constraint.set_motor_target(Quat::IDENTITY);
```

## 万向节约束 (Universal)

允许两个独立的旋转轴。

```rust
let constraint = unsafe { UniversalConstraint::new(
    body_a.handle(),
    body_b.handle(),
    Vec3::new(0.0, 0.0, 0.0),  // 锚点
    Vec3::new(1.0, 0.0, 0.0),  // 轴1
    Vec3::new(0.0, 1.0, 0.0),  // 轴2
) };

constraint.set_breaking_impulse_threshold(100.0);
```

## 双铰链约束 (Hinge2)

类似万向节，但用于车辆悬挂。

```rust
let constraint = unsafe { Hinge2Constraint::new(
    body_a.handle(),
    body_b.handle(),
    Vec3::new(0.0, 0.0, 0.0),  // 锚点
    Vec3::new(0.0, 1.0, 0.0),  // 轴1（悬挂）
    Vec3::new(1.0, 0.0, 0.0),  // 轴2（转向）
) };
```

## 齿轮约束 (Gear)

使两个刚体的旋转联动。

```rust
let constraint = unsafe { GearConstraint::new(
    body_a.handle(),
    body_b.handle(),
    Vec3::new(0.0, 1.0, 0.0),  // 轴 A
    Vec3::new(0.0, 1.0, 0.0),  // 轴 B
) };
```

## 约束属性

### 启用/禁用

```rust
constraint.set_enabled(true);
let enabled = constraint.is_enabled();
```

### 断裂阈值

```rust
// 设置断裂冲量阈值
constraint.set_breaking_impulse_threshold(100.0);

// 获取阈值
let threshold = constraint.get_breaking_impulse_threshold();
```

## ConstraintBuilder 完整 API

```rust
impl ConstraintBuilder {
    pub fn new() -> Self;
    
    // 约束类型
    pub fn point2point(self, body_a: *mut c_void, body_b: *mut c_void) -> Self;
    pub fn generic_6dof(self, body_a: *mut c_void, body_b: *mut c_void) -> Self;
    pub fn generic_6dof_spring(self, body_a: *mut c_void, body_b: *mut c_void) -> Self;
    pub fn cone_twist(self, body_a: *mut c_void, body_b: *mut c_void) -> Self;
    
    // 锚点
    pub fn pivot_a(self, pivot: Vec3) -> Self;
    pub fn pivot_b(self, pivot: Vec3) -> Self;
    
    // 变换帧
    pub fn frame_a(self, frame: Transform) -> Self;
    pub fn frame_b(self, frame: Transform) -> Self;
    
    // 限制
    pub fn linear_limits(self, lower: Vec3, upper: Vec3) -> Self;
    pub fn angular_limits(self, lower: Vec3, upper: Vec3) -> Self;
    
    // 断裂
    pub fn breaking_impulse_threshold(self, threshold: Real) -> Self;
    
    // 弹簧
    pub fn spring(self, axis: i32, stiffness: Real, damping: Real) -> Self;
    
    // 圆锥扭转
    pub fn cone_twist_limit(
        self,
        swing_span1: Real,
        swing_span2: Real,
        twist_span: Real,
        softness: Real,
        bias_factor: Real,
        relaxation_factor: Real,
    ) -> Self;
    
    // 构建
    pub fn build(self) -> Result<*mut c_void, &'static str>;
}
```

## 完整示例

### 钟摆

```rust
use nekobullet::*;

fn main() {
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 固定点
    let anchor_shape = ShapeHandle::new_sphere(0.1);
    let anchor = RigidBodyBuilder::new()
        .shape(anchor_shape)
        .mass(0.0)  // 静态
        .position(Vec3::new(0.0, 10.0, 0.0))
        .build()
        .unwrap();
    let anchor_handle = world.add_rigid_body(anchor);

    // 钟摆球
    let ball_shape = ShapeHandle::new_sphere(0.5);
    let ball = RigidBodyBuilder::new()
        .shape(ball_shape)
        .mass(1.0)
        .position(Vec3::new(2.0, 10.0, 0.0))
        .build()
        .unwrap();
    let ball_handle = world.add_rigid_body(ball);

    // 创建约束
    world.create_point2point_constraint(
        anchor_handle,
        ball_handle,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, 0.0, 0.0),
    );

    // 模拟
    for i in 0..240 {
        world.step(1.0 / 60.0);
        
        let pos = world.get_rigid_body(ball_handle).unwrap().get_position();
        println!("Frame {}: x = {:.2}", i, pos.x);
    }
}
```

### 链条

```rust
use nekobullet::*;

fn main() {
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    let shape = ShapeHandle::new_sphere(0.2);
    let mut handles = Vec::new();

    // 创建固定点
    let anchor = RigidBodyBuilder::new()
        .shape(shape.clone())
        .mass(0.0)
        .position(Vec3::new(0.0, 10.0, 0.0))
        .build()
        .unwrap();
    handles.push(world.add_rigid_body(anchor));

    // 创建链条
    for i in 1..=10 {
        let ball = RigidBodyBuilder::new()
            .shape(shape.clone())
            .mass(1.0)
            .position(Vec3::new(0.0, 10.0 - i as f32 * 0.5, 0.0))
            .build()
            .unwrap();
        let handle = world.add_rigid_body(ball);
        
        // 连接到上一个球
        world.create_point2point_constraint(
            handles[i - 1],
            handle,
            Vec3::new(0.0, -0.2, 0.0),
            Vec3::new(0.0, 0.2, 0.0),
        );
        
        handles.push(handle);
    }

    // 模拟
    for _ in 0..240 {
        world.step(1.0 / 60.0);
    }
}
```

## 性能提示

1. **减少约束数量**：约束会增加计算开销
2. **使用适当的迭代次数**：通过 `solver_iterations` 调整
3. **设置合理的断裂阈值**：避免约束过度拉伸
4. **使用正确的约束类型**：选择最适合的约束类型
