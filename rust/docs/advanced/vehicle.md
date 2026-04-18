# Vehicle

车辆系统提供完整的轮式车辆物理模拟。

## VehicleTuning

车辆调校参数。

```rust
use nekobullet::*;

let tuning = VehicleTuning {
    suspension_stiffness: 5.88,      // 悬挂刚度
    suspension_compression: 0.83,    // 悬挂压缩
    suspension_damping: 0.88,        // 悬挂阻尼
    max_suspension_travel_cm: 500.0, // 最大悬挂行程（厘米）
    friction_slip: 10.5,             // 摩擦滑移
    max_suspension_force: 6000.0,    // 最大悬挂力
};

// 使用默认值
let tuning = VehicleTuning::default();

// 使用 Bullet 原生默认调参（与 C++ 侧保持一致）
let native_tuning = VehicleTuning::from_native_default();
```

## VehicleRaycaster

射线投射器，用于检测车轮与地面的接触。

```rust
let raycaster = VehicleRaycaster::new(&world);
```

## Vehicle

车辆对象。

### 创建

```rust
// 创建底盘刚体
let chassis_shape = ShapeHandle::new_box(Vec3::new(1.0, 0.5, 2.0));
let chassis = RigidBodyBuilder::new()
    .shape(chassis_shape)
    .mass(1500.0)
    .position(Vec3::new(0.0, 1.0, 0.0))
    .build()
    .unwrap();
let chassis_handle = world.add_rigid_body(chassis);

// 创建车辆
let raycaster = VehicleRaycaster::new(&world);
let tuning = VehicleTuning::default();

let mut vehicle = Vehicle::new(
    world.get_rigid_body(chassis_handle).unwrap(),
    &raycaster,
    &tuning,
);
```

### 添加车轮

```rust
// 添加左前轮
let wheel_index = vehicle.add_wheel(
    Vec3::new(1.0, 0.0, 1.5),   // 连接点（局部坐标）
    Vec3::new(0.0, -1.0, 0.0),  // 悬挂方向
    Vec3::new(-1.0, 0.0, 0.0),  // 车轴方向
    0.3,   // 悬挂静止长度
    0.4,   // 车轮半径
    true,  // 是否为前轮
    &tuning,
);

// 添加右前轮
vehicle.add_wheel(
    Vec3::new(-1.0, 0.0, 1.5),
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
    0.3,
    0.4,
    true,
    &tuning,
);

// 添加左后轮
vehicle.add_wheel(
    Vec3::new(1.0, 0.0, -1.5),
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(-1.0, 0.0, 0.0),
    0.3,
    0.4,
    false,
    &tuning,
);

// 添加右后轮
vehicle.add_wheel(
    Vec3::new(-1.0, 0.0, -1.5),
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
    0.3,
    0.4,
    false,
    &tuning,
);
```

### 车轮信息

```rust
// 获取车轮数量
let num_wheels = vehicle.num_wheels();

// 获取车轮半径
let radius = vehicle.wheel_radius(0);

// 获取悬挂长度
let suspension_length = vehicle.wheel_suspension_length(0);

// 检查车轮是否接触地面
if vehicle.is_wheel_in_contact(0) {
    let contact_normal = vehicle.wheel_contact_normal(0);
    let contact_point = vehicle.wheel_contact_point(0);
}

// 获取车轮变换
let transform = vehicle.wheel_transform(0);
```

### 控制

#### 转向

```rust
// 设置转向角度（弧度）
vehicle.set_steering(0.3, 0);  // 车轮索引 0
vehicle.set_steering(0.3, 1);  // 车轮索引 1

// 获取转向角度
let steering = vehicle.steering(0);
```

#### 引擎力

```rust
// 施加引擎力（正值为前进，负值为后退）
vehicle.apply_engine_force(500.0, 2);  // 后轮驱动
vehicle.apply_engine_force(500.0, 3);
```

#### 刹车

```rust
// 设置刹车力
vehicle.set_brake(100.0, 0);
vehicle.set_brake(100.0, 1);
vehicle.set_brake(100.0, 2);
vehicle.set_brake(100.0, 3);
```

### 车辆状态

```rust
// 获取当前速度（公里/小时）
let speed = vehicle.current_speed_km_hour();

// 获取前进方向
let forward = vehicle.forward_vector();
```

### 坐标系统

```rust
// 设置坐标系统
// right: 右轴索引
// up: 上轴索引
// forward: 前轴索引
vehicle.set_coordinate_system(0, 1, 2);
```

### 重置

```rust
// 重置悬挂
vehicle.reset_suspension();

// 更新车轮变换
vehicle.update_wheel_transform(0, true);  // interpolated
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
        .friction(1.0)
        .build()
        .unwrap();
    world.add_rigid_body(ground);

    // 创建底盘
    let chassis_shape = ShapeHandle::new_box(Vec3::new(1.0, 0.5, 2.0));
    let chassis = RigidBodyBuilder::new()
        .shape(chassis_shape)
        .mass(1500.0)
        .position(Vec3::new(0.0, 1.0, 0.0))
        .build()
        .unwrap();
    let chassis_handle = world.add_rigid_body(chassis);

    // 创建车辆
    let raycaster = VehicleRaycaster::new(&world);
    let tuning = VehicleTuning::default();

    let mut vehicle = Vehicle::new(
        world.get_rigid_body(chassis_handle).unwrap(),
        &raycaster,
        &tuning,
    );

    // 添加车轮
    let wheel_positions = [
        (Vec3::new(1.0, 0.0, 1.5), true),   // 左前
        (Vec3::new(-1.0, 0.0, 1.5), true),  // 右前
        (Vec3::new(1.0, 0.0, -1.5), false), // 左后
        (Vec3::new(-1.0, 0.0, -1.5), false),// 右后
    ];

    for (pos, is_front) in wheel_positions {
        vehicle.add_wheel(
            pos,
            Vec3::new(0.0, -1.0, 0.0),
            if pos.x > 0.0 { Vec3::new(-1.0, 0.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) },
            0.3,
            0.4,
            is_front,
            &tuning,
        );
    }

    world.add_vehicle(&vehicle);

    // 模拟
    let mut steering = 0.0;
    let mut engine_force = 0.0;

    for i in 0..600 {
        // 简单控制逻辑
        if i < 100 {
            engine_force = 500.0;  // 加速
        } else if i < 200 {
            steering = 0.3;  // 左转
            engine_force = 300.0;
        } else if i < 300 {
            steering = -0.3; // 右转
            engine_force = 300.0;
        } else {
            steering = 0.0;  // 直行
            engine_force = 0.0;  // 滑行
        }

        // 应用控制
        vehicle.set_steering(steering, 0);
        vehicle.set_steering(steering, 1);
        vehicle.apply_engine_force(engine_force, 2);
        vehicle.apply_engine_force(engine_force, 3);

        // 模拟
        world.step(1.0 / 60.0);

        // 输出状态
        let speed = vehicle.current_speed_km_hour();
        let pos = world.get_rigid_body(chassis_handle).unwrap().get_position();
        
        if i % 10 == 0 {
            println!("Frame {}: speed = {:.1} km/h, pos = ({:.2}, {:.2}, {:.2})",
                     i, speed, pos.x, pos.y, pos.z);
        }
    }
}
```

## 性能提示

1. **合理设置质量**：底盘质量影响车辆行为
2. **调整悬挂参数**：根据车辆类型调整悬挂
3. **使用适当的摩擦系数**：地面摩擦影响抓地力
4. **控制更新频率**：车辆控制不需要每帧更新
