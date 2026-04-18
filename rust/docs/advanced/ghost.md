# GhostObject

Ghost 对象用于碰撞检测而不产生物理响应，常用于触发区域、传感器、AI 感知等场景。

## 创建

### 基本创建

```rust
use nekobullet::*;

let ghost = GhostObject::new();
```

### 使用构建器

```rust
let ghost = GhostObjectBuilder::new()
    .shape(ShapeHandle::new_box(Vec3::new(1.0, 1.0, 1.0)))
    .position(Vec3::new(0.0, 5.0, 0.0))
    .rotation(Quat::IDENTITY)
    .build()
    .unwrap();
```

## 设置形状

```rust
let shape = ShapeHandle::new_box(Vec3::new(1.0, 1.0, 1.0));
ghost.set_shape(shape);

// 获取形状
if let Some(shape) = ghost.get_shape() {
    let shape_type = shape.shape_type();
}
```

## 变换

### 位置

```rust
// 设置位置
ghost.set_position(Vec3::new(0.0, 5.0, 0.0));

// 获取位置
let pos = ghost.get_position();
```

### 旋转

```rust
// 设置旋转
ghost.set_rotation(Quat::from_rotation_y(std::f32::consts::PI));

// 获取旋转
let rot = ghost.get_rotation();
```

### 完整变换

```rust
// 设置完整变换
ghost.set_transform(&Transform::new(
    Vec3::new(0.0, 5.0, 0.0),
    Quat::from_rotation_y(std::f32::consts::PI),
));

// 获取完整变换
let transform = ghost.get_transform();
```

## 重叠检测

### 获取重叠对象数量

```rust
let count = ghost.get_num_overlapping_objects();
println!("Overlapping with {} objects", count);
```

### 获取单个重叠对象

```rust
if let Some(obj) = ghost.get_overlapping_object(0) {
    println!("Overlapping with object: {:?}", obj);
}
```

### 遍历重叠对象

```rust
for obj in ghost.overlapping_objects() {
    println!("Overlapping with: {:?}", obj);
}
```

## 添加到世界

```rust
// 添加 Ghost 对象到世界
let ghost_handle = world.add_ghost(ghost);

// 获取 Ghost 对象
if let Some(ghost) = world.get_ghost(ghost_handle) {
    let count = ghost.get_num_overlapping_objects();
}

// 移除 Ghost 对象
world.remove_ghost(ghost_handle);
```

## 完整示例

### 触发区域

```rust
use nekobullet::*;

fn main() {
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 创建地面
    let ground_shape = ShapeHandle::new_box(Vec3::new(50.0, 0.5, 50.0));
    let ground = RigidBodyBuilder::new()
        .shape(ground_shape)
        .mass(0.0)
        .build()
        .unwrap();
    world.add_rigid_body(ground);

    // 创建触发区域
    let trigger_shape = ShapeHandle::new_box(Vec3::new(2.0, 2.0, 2.0));
    let mut trigger = GhostObject::new();
    trigger.set_shape(trigger_shape);
    trigger.set_position(Vec3::new(0.0, 2.0, 0.0));
    
    let trigger_handle = world.add_ghost(trigger);

    // 创建一些球
    let ball_shape = ShapeHandle::new_sphere(0.5);
    for i in 0..5 {
        let ball = RigidBodyBuilder::new()
            .shape(ball_shape.clone())
            .mass(1.0)
            .position(Vec3::new(i as f32 - 2.0, 5.0, 0.0))
            .build()
            .unwrap();
        world.add_rigid_body(ball);
    }

    // 模拟
    for i in 0..120 {
        world.step(1.0 / 60.0);

        // 检查触发区域
        if let Some(trigger) = world.get_ghost(trigger_handle) {
            let count = trigger.get_num_overlapping_objects();
            if count > 0 {
                println!("Frame {}: {} objects in trigger zone", i, count);
            }
        }
    }
}
```

### 传感器

```rust
use nekobullet::*;

struct ProximitySensor {
    ghost: GhostObject,
    detection_range: f32,
}

impl ProximitySensor {
    fn new(position: Vec3, range: f32) -> Self {
        let mut ghost = GhostObject::new();
        ghost.set_shape(ShapeHandle::new_sphere(range));
        ghost.set_position(position);

        Self {
            ghost,
            detection_range: range,
        }
    }

    fn detect(&self) -> i32 {
        self.ghost.get_num_overlapping_objects()
    }

    fn detected_objects(&self) -> impl Iterator<Item = *mut std::ffi::c_void> + '_ {
        self.ghost.overlapping_objects()
    }
}

fn main() {
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 创建传感器
    let sensor = ProximitySensor::new(Vec3::new(0.0, 1.0, 0.0), 5.0);
    let ghost_handle = world.add_ghost(sensor.ghost);

    // 模拟
    loop {
        world.step(1.0 / 60.0);

        if let Some(ghost) = world.get_ghost(ghost_handle) {
            let count = ghost.get_num_overlapping_objects();
            if count > 0 {
                println!("Detected {} objects", count);
            }
        }
    }
}
```

### AI 感知区域

```rust
use nekobullet::*;

struct AIVisionCone {
    ghost: GhostObject,
    view_distance: f32,
    view_angle: f32,
}

impl AIVisionCone {
    fn new(position: Vec3, direction: Vec3, view_distance: f32, view_angle: f32) -> Self {
        // 创建锥形碰撞形状（使用凸包近似）
        let mut points = Vec::new();
        let segments = 8;
        
        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let x = angle.cos() * view_distance * (view_angle / 2.0).tan();
            let z = angle.sin() * view_distance * (view_angle / 2.0).tan();
            points.push(Vec3::new(x, view_distance, z));
        }
        points.push(Vec3::ZERO);  // 顶点

        let mut ghost = GhostObject::new();
        ghost.set_shape(ShapeHandle::new_convex_hull(&points));
        ghost.set_position(position);

        // 设置旋转朝向方向
        let rotation = Quat::from_rotation_arc(Vec3::new(0.0, 1.0, 0.0), direction.normalize());
        ghost.set_rotation(rotation);

        Self {
            ghost,
            view_distance,
            view_angle,
        }
    }

    fn can_see(&self) -> bool {
        self.ghost.get_num_overlapping_objects() > 0
    }
}
```

## GhostObjectBuilder API

```rust
impl GhostObjectBuilder {
    pub fn new() -> Self;
    
    pub fn shape(self, shape: ShapeHandle) -> Self;
    pub fn position(self, position: Vec3) -> Self;
    pub fn rotation(self, rotation: Quat) -> Self;
    
    pub fn build(self) -> Result<GhostObject, &'static str>;
}
```

## GhostObject API

```rust
impl GhostObject {
    pub fn new() -> Self;
    
    // 形状
    pub fn set_shape(&mut self, shape: ShapeHandle);
    pub fn get_shape(&self) -> Option<&ShapeHandle>;
    
    // 变换
    pub fn set_transform(&self, transform: &Transform);
    pub fn get_transform(&self) -> Transform;
    pub fn set_position(&self, position: Vec3);
    pub fn get_position(&self) -> Vec3;
    pub fn set_rotation(&self, rotation: Quat);
    pub fn get_rotation(&self) -> Quat;
    
    // 重叠检测
    pub fn get_num_overlapping_objects(&self) -> i32;
    pub fn get_overlapping_object(&self, index: i32) -> Option<*mut c_void>;
    pub fn overlapping_objects(&self) -> OverlappingObjectsIter<'_>;
}
```

## 性能提示

1. **使用简单形状**：简单的碰撞形状检测更快
2. **合理设置检测频率**：不需要每帧都检测
3. **使用碰撞过滤**：只检测需要的对象
4. **及时移除**：不再需要的 Ghost 对象应及时移除
