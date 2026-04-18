# ReducedDeformableBody

简化软体是一种基于模态分析的软体模拟方法，通过减少自由度来提高性能，同时保持视觉上的软体效果。

## 创建

```rust
use nekobullet::*;

// 创建简化软体
let reduced_body = ReducedDeformableBody::new();
```

## 设置模态

```rust
// 设置模态数量
reduced_body.set_num_modes(10);

// 设置模态参数
// modes: 模态向量（每个模态是一个位移场）
// frequencies: 模态频率
let modes: Vec<Vec3> = vec![/* 模态向量 */];
let frequencies: Vec<f32> = vec![/* 频率 */];

reduced_body.set_modes(&modes, &frequencies);
```

## 刚体变换

```rust
// 获取刚体变换
let transform = reduced_body.get_rigid_transform();
let position = transform.position;
let rotation = transform.rotation;

// 设置刚体变换
reduced_body.set_rigid_transform(&Transform::new(
    Vec3::new(0.0, 5.0, 0.0),
    Quat::IDENTITY,
));
```

## 节点位置

```rust
// 获取节点数量
let num_nodes = reduced_body.num_nodes();

// 获取节点位置（世界坐标）
for i in 0..num_nodes {
    let pos = reduced_body.get_node_position(i);
    println!("Node {}: {:?}", i, pos);
}

// 获取节点位置（局部坐标）
for i in 0..num_nodes {
    let local_pos = reduced_body.get_node_local_position(i);
    println!("Node {} local: {:?}", i, local_pos);
}
```

## 模态坐标

```rust
// 获取模态坐标
let modal_coords = reduced_body.get_modal_coordinates();

// 设置模态坐标
reduced_body.set_modal_coordinates(&modal_coords);
```

## 阻尼

```rust
// 设置阻尼系数
reduced_body.set_damping(0.1);

// 获取阻尼系数
let damping = reduced_body.damping();
```

## 质量

```rust
// 获取总质量
let mass = reduced_body.total_mass();

// 设置总质量
reduced_body.set_total_mass(1.0);
```

## 完整示例

```rust
use nekobullet::*;

fn main() {
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    // 创建简化软体
    let reduced_body = ReducedDeformableBody::new();
    
    // 设置基本参数
    reduced_body.set_total_mass(1.0);
    reduced_body.set_damping(0.05);
    reduced_body.set_num_modes(5);
    
    // 设置初始位置
    reduced_body.set_rigid_transform(&Transform::from_position(
        Vec3::new(0.0, 5.0, 0.0)
    ));

    // 添加到世界
    world.add_softbody(&reduced_body);

    // 模拟
    for i in 0..120 {
        world.step(1.0 / 60.0);
        
        // 获取刚体变换
        let transform = reduced_body.get_rigid_transform();
        
        // 获取节点位置
        if reduced_body.num_nodes() > 0 {
            let pos = reduced_body.get_node_position(0);
            println!("Frame {}: y = {:.2}", i, pos.y);
        }
    }
}
```

## 与 SoftBody 的比较

| 特性 | ReducedDeformableBody | SoftBody |
|------|----------------------|----------|
| 性能 | 高 | 中等 |
| 精度 | 视觉近似 | 物理精确 |
| 适用场景 | 实时游戏 | 物理模拟 |
| 自由度 | 可配置 | 节点数量 |
| 变形范围 | 小变形 | 大变形 |

## ReducedDeformableBody API

```rust
impl ReducedDeformableBody {
    pub fn new() -> Self;
    pub fn handle(&self) -> *mut c_void;
    
    // 模态
    pub fn set_num_modes(&self, num: i32);
    pub fn set_modes(&self, modes: &[Vec3], frequencies: &[Real]);
    
    // 变换
    pub fn get_rigid_transform(&self) -> Transform;
    pub fn set_rigid_transform(&self, transform: &Transform);
    
    // 节点
    pub fn num_nodes(&self) -> i32;
    pub fn get_node_position(&self, index: i32) -> Vec3;
    pub fn get_node_local_position(&self, index: i32) -> Vec3;
    
    // 模态坐标
    pub fn get_modal_coordinates(&self) -> Vec<Real>;
    pub fn set_modal_coordinates(&self, coords: &[Real]);
    
    // 物理属性
    pub fn damping(&self) -> Real;
    pub fn set_damping(&self, damping: Real);
    pub fn total_mass(&self) -> Real;
    pub fn set_total_mass(&self, mass: Real);
}
```

## 性能提示

1. **选择合适的模态数量**：模态越多越精确，但性能越低
2. **预计算模态**：模态可以在离线预计算
3. **适用场景**：适合小变形的软体效果，如布料、果冻
4. **与刚体结合**：可以与刚体物理结合使用
