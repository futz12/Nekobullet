# CharacterController

角色控制器用于游戏角色的物理移动，提供碰撞检测、步高处理、跳跃等功能。

## 创建

```rust
use nekobullet::*;

// 创建 Ghost 对象
let mut ghost = GhostObject::new();

// 设置碰撞形状（通常使用胶囊）
let shape = ShapeHandle::new_capsule(0.3, 1.0);
ghost.set_shape(shape);
ghost.set_position(Vec3::new(0.0, 1.0, 0.0));

// 创建角色控制器
let character = CharacterController::new(
    &ghost,
    &shape,
    0.3,  // 步高
    Vec3::new(0.0, 1.0, 0.0),  // 上方向
);
```

## 移动控制

### 行走方向

```rust
// 设置行走方向
character.set_walk_direction(Vec3::new(0.0, 0.0, 1.0));

// 设置速度和时间
character.set_velocity_for_time(
    Vec3::new(0.0, 0.0, 5.0),  // 速度
    1.0 / 60.0,  // 时间间隔
);
```

### 线速度

```rust
// 设置线速度
character.set_linear_velocity(Vec3::new(0.0, 0.0, 5.0));

// 获取线速度
let vel = character.linear_velocity();
```

### 角速度

```rust
// 设置角速度
character.set_angular_velocity(Vec3::new(0.0, 1.0, 0.0));

// 获取角速度
let vel = character.angular_velocity();
```

## 跳跃

```rust
// 检查是否可以跳跃
if character.can_jump() {
    // 执行跳跃
    character.jump(Vec3::new(0.0, 5.0, 0.0));  // 跳跃速度
}

// 检查是否在地面
if character.on_ground() {
    println!("On ground");
}

// 设置跳跃速度
character.set_jump_speed(5.0);
let jump_speed = character.jump_speed();

// 设置最大跳跃高度
character.set_max_jump_height(2.0);
```

## 下落

```rust
// 设置下落速度
character.set_fall_speed(10.0);
let fall_speed = character.fall_speed();
```

## 斜坡限制

```rust
// 设置最大斜坡角度（弧度）
character.set_max_slope(std::f32::consts::FRAC_PI_4);  // 45度

// 获取最大斜坡角度
let max_slope = character.max_slope();
```

## 步高

```rust
// 设置步高
character.set_step_height(0.3);

// 获取步高
let step_height = character.step_height();
```

## 重力

```rust
// 设置重力
character.set_gravity(Vec3::new(0.0, -9.81, 0.0));

// 获取重力
let gravity = character.gravity();
```

## 上方向

```rust
// 设置上方向
character.set_up(Vec3::new(0.0, 1.0, 0.0));

// 获取上方向
let up = character.up();
```

## 阻尼

```rust
// 设置线性阻尼
character.set_linear_damping(0.1);
let linear_damping = character.linear_damping();

// 设置角阻尼
character.set_angular_damping(0.1);
let angular_damping = character.angular_damping();
```

## 穿透深度

```rust
// 设置最大穿透深度
character.set_max_penetration_depth(0.1);

// 获取最大穿透深度
let depth = character.max_penetration_depth();
```

## 位置控制

```rust
// 瞬移到指定位置
character.warp(Vec3::new(0.0, 5.0, 0.0));

// 重置状态
character.reset();
```

## Ghost 对象

```rust
// 获取关联的 Ghost 对象
let ghost_ptr = character.ghost_object();
```

## 完整示例

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
        .position(Vec3::new(0.0, -0.5, 0.0))
        .build()
        .unwrap();
    world.add_rigid_body(ground);

    // 创建一些障碍物
    let obstacle_shape = ShapeHandle::new_box(Vec3::new(1.0, 0.5, 1.0));
    for i in 0..5 {
        let obstacle = RigidBodyBuilder::new()
            .shape(obstacle_shape.clone())
            .mass(0.0)
            .position(Vec3::new(i as f32 * 3.0, 0.25, 5.0))
            .build()
            .unwrap();
        world.add_rigid_body(obstacle);
    }

    // 创建角色
    let mut ghost = GhostObject::new();
    let shape = ShapeHandle::new_capsule(0.3, 1.0);
    ghost.set_shape(shape);
    ghost.set_position(Vec3::new(0.0, 1.0, 0.0));

    let character = CharacterController::new(
        &ghost,
        &ShapeHandle::new_capsule(0.3, 1.0),
        0.3,
        Vec3::new(0.0, 1.0, 0.0),
    );

    world.add_ghost(&ghost);
    world.add_character(&character);

    // 模拟
    let mut jump_requested = false;
    let mut move_direction = Vec3::ZERO;

    for i in 0..600 {
        // 简单的输入模拟
        if i < 100 {
            move_direction = Vec3::new(0.0, 0.0, 1.0);  // 前进
        } else if i < 200 {
            move_direction = Vec3::new(1.0, 0.0, 0.0);  // 右移
        } else if i < 300 {
            move_direction = Vec3::new(0.0, 0.0, -1.0); // 后退
        } else {
            move_direction = Vec3::new(-1.0, 0.0, 0.0); // 左移
        }

        // 跳跃
        if i % 60 == 30 && character.on_ground() {
            character.jump(Vec3::new(0.0, 5.0, 0.0));
        }

        // 设置移动
        character.set_walk_direction(move_direction * 5.0);

        // 模拟
        world.step(1.0 / 60.0);

        // 输出状态
        if i % 10 == 0 {
            let pos = ghost.get_position();
            let vel = character.linear_velocity();
            println!("Frame {}: pos=({:.2}, {:.2}, {:.2}), on_ground={}",
                     i, pos.x, pos.y, pos.z, character.on_ground());
        }
    }
}
```

## 第一人称控制器示例

```rust
use nekobullet::*;

struct FirstPersonController {
    ghost: GhostObject,
    character: CharacterController,
    yaw: f32,
    pitch: f32,
}

impl FirstPersonController {
    fn new(position: Vec3) -> Self {
        let mut ghost = GhostObject::new();
        let shape = ShapeHandle::new_capsule(0.3, 1.0);
        ghost.set_shape(shape);
        ghost.set_position(position);

        let character = CharacterController::new(
            &ghost,
            &ShapeHandle::new_capsule(0.3, 1.0),
            0.3,
            Vec3::new(0.0, 1.0, 0.0),
        );

        Self {
            ghost,
            character,
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    fn update(&mut self, forward: f32, right: f32, jump: bool, dt: f32) {
        // 计算移动方向
        let forward_dir = Vec3::new(
            self.yaw.sin(),
            0.0,
            self.yaw.cos(),
        );
        let right_dir = Vec3::new(
            self.yaw.cos(),
            0.0,
            -self.yaw.sin(),
        );

        let move_dir = forward_dir * forward + right_dir * right;
        
        if move_dir.length() > 0.0 {
            self.character.set_walk_direction(move_dir.normalize() * 5.0);
        } else {
            self.character.set_walk_direction(Vec3::ZERO);
        }

        // 跳跃
        if jump && self.character.on_ground() {
            self.character.jump(Vec3::new(0.0, 5.0, 0.0));
        }
    }

    fn position(&self) -> Vec3 {
        self.ghost.get_position()
    }
}
```

## 性能提示

1. **使用胶囊形状**：胶囊形状适合角色碰撞
2. **合理设置步高**：过高的步高可能导致穿墙
3. **设置适当的斜坡限制**：防止角色爬上不可能的斜坡
4. **使用 Ghost 对象**：角色控制器使用 Ghost 对象进行碰撞检测
