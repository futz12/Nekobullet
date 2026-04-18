# MultiBodyConstraint

多体约束用于 Featherstone 算法的多体系统，提供高效的关节约束。

## 约束类型常量

```rust
use nekobullet::*;

// 约束类型
pub const CONSTRAINT_POINT2POINT: i32 = 0;
pub const CONSTRAINT_FIXED: i32 = 1;
pub const CONSTRAINT_SLIDER: i32 = 2;
```

## MultiBodyPoint2PointConstraint

点对点约束，将多体系统的某个物体锚定到世界空间点。

### 创建

```rust
use nekobullet::*;

// 创建点对点约束
let constraint = MultiBodyPoint2PointConstraint::new(
    &multibody,
    0,      // link_index（-1 表示基座）
    Vec3::new(0.0, 1.0, 0.0),  // pivot_in_world
    Vec3::new(0.0, 0.0, 0.0),  // pivot_in_link
);

// 完成
constraint.finalize();
```

### 属性

```rust
// 获取句柄
let handle = constraint.handle();

// 获取多体系统
let multibody = constraint.multibody();

// 获取链接索引
let link = constraint.link_index();

// 获取锚点
let pivot = constraint.pivot_in_world();
let pivot_local = constraint.pivot_in_link();
```

## MultiBodyFixedConstraint

固定约束，将多体系统的某个物体固定到世界空间。

### 创建

```rust
let constraint = MultiBodyFixedConstraint::new(
    &multibody,
    0,      // link_index
    Transform::identity(),  // frame_in_world
    Transform::identity(),  // frame_in_link
);

constraint.finalize();
```

### 属性

```rust
let handle = constraint.handle();
let multibody = constraint.multibody();
let link = constraint.link_index();
```

## MultiBodySliderConstraint

滑动约束，允许沿轴滑动。

### 创建

```rust
let constraint = MultiBodySliderConstraint::new(
    &multibody,
    0,      // link_index
    Transform::identity(),  // frame_in_world
    Transform::identity(),  // frame_in_link
);

constraint.finalize();
```

### 属性

```rust
let handle = constraint.handle();
let multibody = constraint.multibody();
let link = constraint.link_index();
```

## 设置冲量限制

```rust
// 设置最大冲量
constraint.set_max_impulse(100.0);

// 获取最大冲量
let max_impulse = constraint.max_impulse();
```

## 完整示例

### 机械臂约束

```rust
use nekobullet::*;

fn main() {
    // 创建多体系统
    let mut multibody = MultiBody::new();

    // 添加基座
    multibody.add_body(
        0, -1,
        JointType::Fixed,
        Vec3::ZERO,
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        Vec3::new(0.0, 1.0, 0.0),
        10.0,
        Vec3::new(0.0, 0.5, 0.0),
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    );

    // 添加第一关节
    multibody.add_body(
        1, 0,
        JointType::Revolute,
        Vec3::new(0.0, 1.0, 0.0),
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        Vec3::new(0.0, 1.0, 0.0),
        5.0,
        Vec3::new(0.0, 0.5, 0.0),
        [[0.1, 0.0, 0.0], [0.0, 0.1, 0.0], [0.0, 0.0, 0.1]],
    );

    multibody.finalize();

    // 创建约束：将末端固定在世界空间
    let constraint = MultiBodyFixedConstraint::new(
        &multibody,
        1,  // link_index
        Transform::from_position(Vec3::new(0.0, 2.0, 0.0)),
        Transform::from_position(Vec3::new(0.0, 1.0, 0.0)),
    );

    constraint.set_max_impulse(50.0);
    constraint.finalize();

    println!("Constraint created with max impulse: {}", constraint.max_impulse());
}
```

### 点对点约束

```rust
use nekobullet::*;

fn main() {
    let mut multibody = MultiBody::new();

    // 创建简单的多体系统
    multibody.add_body(
        0, -1,
        JointType::Floating,
        Vec3::ZERO,
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Vec3::ZERO,
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    );

    multibody.finalize();

    // 创建点对点约束
    let constraint = MultiBodyPoint2PointConstraint::new(
        &multibody,
        0,  // link_index
        Vec3::new(0.0, 5.0, 0.0),  // 世界空间锚点
        Vec3::new(0.0, 0.0, 0.0),  // 局部空间锚点
    );

    constraint.finalize();

    println!("Point2Point constraint created");
}
```

## MultiBodyPoint2PointConstraint API

```rust
impl MultiBodyPoint2PointConstraint {
    pub fn new(
        multibody: &MultiBody,
        link_index: i32,
        pivot_in_world: Vec3,
        pivot_in_link: Vec3,
    ) -> Self;
    
    pub fn handle(&self) -> *mut c_void;
    pub fn multibody(&self) -> &MultiBody;
    pub fn link_index(&self) -> i32;
    pub fn pivot_in_world(&self) -> Vec3;
    pub fn pivot_in_link(&self) -> Vec3;
    pub fn set_max_impulse(&self, max: Real);
    pub fn max_impulse(&self) -> Real;
    pub fn finalize(&self);
}
```

## MultiBodyFixedConstraint API

```rust
impl MultiBodyFixedConstraint {
    pub fn new(
        multibody: &MultiBody,
        link_index: i32,
        frame_in_world: Transform,
        frame_in_link: Transform,
    ) -> Self;
    
    pub fn handle(&self) -> *mut c_void;
    pub fn multibody(&self) -> &MultiBody;
    pub fn link_index(&self) -> i32;
    pub fn set_max_impulse(&self, max: Real);
    pub fn max_impulse(&self) -> Real;
    pub fn finalize(&self);
}
```

## MultiBodySliderConstraint API

```rust
impl MultiBodySliderConstraint {
    pub fn new(
        multibody: &MultiBody,
        link_index: i32,
        frame_in_world: Transform,
        frame_in_link: Transform,
    ) -> Self;
    
    pub fn handle(&self) -> *mut c_void;
    pub fn multibody(&self) -> &MultiBody;
    pub fn link_index(&self) -> i32;
    pub fn set_max_impulse(&self, max: Real);
    pub fn max_impulse(&self) -> Real;
    pub fn finalize(&self);
}
```

## 性能提示

1. **使用正确的约束类型**：选择最适合的约束类型
2. **设置合理的冲量限制**：避免过度约束
3. **调用 finalize**：创建后记得调用 finalize
4. **与 MultiBody 配合**：与逆动力学模块配合使用效果更好
