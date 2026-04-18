# MLCPSolver

MLCP（Mixed Linear Complementarity Problem）求解器提供高级数值求解能力，用于更精确的物理模拟。

## DantzigSolver

Dantzig 算法求解器。

### 创建

```rust
use nekobullet::*;

let solver = DantzigSolver::new();
```

### 属性

```rust
// 获取句柄
let handle = solver.handle();
```

## MLCPSolver

MLCP 求解器。

### 创建

```rust
// 使用 Dantzig 求解器创建
let dantzig = DantzigSolver::new();
let solver = MLCPSolver::new(&dantzig);

// 使用默认求解器创建
let solver = MLCPSolver::new_default();
```

### 属性

```rust
// 获取句柄
let handle = solver.handle();

// 获取回退次数
let fallbacks = solver.num_fallbacks();

// 设置回退次数
solver.set_num_fallbacks(5);
```

## 完整示例

### 基本使用

```rust
use nekobullet::*;

fn main() {
    // 创建 Dantzig 求解器
    let dantzig = DantzigSolver::new();
    
    // 创建 MLCP 求解器
    let solver = MLCPSolver::new(&dantzig);
    
    // 设置回退次数
    solver.set_num_fallbacks(10);
    
    println!("MLCP Solver created");
    println!("Fallbacks: {}", solver.num_fallbacks());
}
```

### 与物理世界集成

```rust
use nekobullet::*;

fn main() {
    // 创建求解器
    let dantzig = DantzigSolver::new();
    let mlcp_solver = MLCPSolver::new(&dantzig);
    
    // 创建物理世界
    let mut world = PhysicsWorld::new();
    world.set_gravity(Vec3::new(0.0, -9.81, 0.0));
    
    // 注意：MLCP 求解器的集成需要通过底层 API
    // 这里展示基本创建流程
    
    // 创建物体
    let shape = ShapeHandle::new_box(Vec3::new(1.0, 1.0, 1.0));
    
    let body = RigidBodyBuilder::new()
        .shape(shape)
        .mass(1.0)
        .position(Vec3::new(0.0, 10.0, 0.0))
        .build()
        .unwrap();
    
    world.add_rigid_body(body);
    
    // 模拟
    for _ in 0..60 {
        world.step(1.0 / 60.0);
    }
}
```

### 高精度模拟

```rust
use nekobullet::*;

struct HighPrecisionSimulation {
    world: PhysicsWorld,
    solver: MLCPSolver,
}

impl HighPrecisionSimulation {
    fn new() -> Self {
        let dantzig = DantzigSolver::new();
        let solver = MLCPSolver::new(&dantzig);
        solver.set_num_fallbacks(20);
        
        let world = PhysicsWorld::new();
        world.set_gravity(Vec3::new(0.0, -9.81, 0.0));
        
        Self { world, solver }
    }
    
    fn step(&mut self, dt: f32) {
        // 使用更小的时间步长
        let sub_steps = 4;
        let sub_dt = dt / sub_steps as f32;
        
        for _ in 0..sub_steps {
            self.world.step(sub_dt);
        }
    }
}

fn main() {
    let mut sim = HighPrecisionSimulation::new();
    
    // 创建物体...
    
    for _ in 0..240 {
        sim.step(1.0 / 60.0);
    }
}
```

## DantzigSolver API

```rust
impl DantzigSolver {
    pub fn new() -> Self;
    pub fn handle(&self) -> *mut c_void;
}
```

## MLCPSolver API

```rust
impl MLCPSolver {
    pub fn new(interface: &DantzigSolver) -> Self;
    pub fn new_default() -> Self;
    pub fn handle(&self) -> *mut c_void;
    pub fn num_fallbacks(&self) -> i32;
    pub fn set_num_fallbacks(&self, num: i32);
}
```

## 使用场景

MLCP 求解器适用于以下场景：

1. **高精度模拟**：需要更精确的物理结果
2. **复杂约束系统**：大量约束的稳定求解
3. **机器人模拟**：精确的关节控制
4. **科学计算**：需要数值稳定性的应用

## 性能提示

1. **调整回退次数**：根据需要设置适当的回退次数
2. **使用子步**：将大时间步分解为多个小步
3. **合理设置约束**：避免过度约束的系统
4. **监控求解器状态**：检查回退次数判断求解质量
