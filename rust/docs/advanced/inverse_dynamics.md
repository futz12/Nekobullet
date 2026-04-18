# InverseDynamics

逆动力学用于计算多体系统所需的关节力矩，常用于机器人控制和动画物理。

## JointType

关节类型。

```rust
use nekobullet::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum JointType {
    Fixed = 0,      // 固定关节
    Revolute = 1,   // 旋转关节（1自由度）
    Prismatic = 2,  // 滑动关节（1自由度）
    Floating = 3,   // 浮动关节（6自由度）
    Spherical = 4,  // 球关节（3自由度）
}
```

## MultiBody

多体系统。

### 创建

```rust
let mut multibody = MultiBody::new();
```

### 添加物体

```rust
multibody.add_body(
    0,      // body_index（从0开始）
    -1,     // parent_index（-1表示根节点）
    JointType::Revolute,  // 关节类型
    Vec3::new(0.0, 1.0, 0.0),  // parent_r（父坐标系中的位置）
    [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],  // body_t_parent（变换矩阵）
    Vec3::new(0.0, 1.0, 0.0),  // axis（关节轴）
    1.0,    // mass
    Vec3::new(0.0, 0.5, 0.0),  // com（质心）
    [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],  // inertia（惯性张量）
);
```

### 完成

```rust
let result = multibody.finalize();
if result == 0 {
    println!("MultiBody finalized successfully");
}

if multibody.is_finalized() {
    println!("MultiBody is ready");
}
```

### 动力学计算

```rust
// 计算逆动力学
let q = vec![0.0; multibody.num_dofs() as usize];      // 关节位置
let u = vec![0.0; multibody.num_dofs() as usize];      // 关节速度
let dot_u = vec![1.0; multibody.num_dofs() as usize]; // 关节加速度
let mut joint_forces = vec![0.0; multibody.num_dofs() as usize];

multibody.calculate_inverse_dynamics(&q, &u, &dot_u, &mut joint_forces);

println!("Joint forces: {:?}", joint_forces);
```

### 质量矩阵

```rust
let q = vec![0.0; multibody.num_dofs() as usize];
let mut mass_matrix = vec![0.0; (multibody.num_dofs() * multibody.num_dofs()) as usize];

multibody.calculate_mass_matrix(
    &q,
    &mut mass_matrix,
    true,   // initialize
    true,   // set_lower_triangular
);
```

### 属性

```rust
// 获取物体数量
let num_bodies = multibody.num_bodies();

// 获取自由度数量
let num_dofs = multibody.num_dofs();

// 设置重力
multibody.set_gravity(Vec3::new(0.0, -9.81, 0.0));

// 设置是否接受无效质量
multibody.set_accept_invalid_mass(true);

// 打印树结构
multibody.print_tree();
```

## MultiBodyNameMap

名称映射，用于给物体和关节命名。

```rust
let mut name_map = MultiBodyNameMap::new();

// 添加名称
name_map.add_body(0, "base");
name_map.add_joint(0, "shoulder");

// 获取名称
if let Some(name) = name_map.get_body_name(0) {
    println!("Body 0 name: {}", name);
}

// 获取索引
if let Some(index) = name_map.get_body_index("base") {
    println!("'base' index: {}", index);
}
```

## MultiBodyTreeCreator

从现有 MultiBody 创建树结构。

```rust
let creator = MultiBodyTreeCreator::from_bt_multibody(&multibody, false).unwrap();
let new_multibody = creator.create_tree().unwrap();
```

## CloneTreeCreator

克隆现有 MultiBody。

```rust
let creator = CloneTreeCreator::new(&multibody).unwrap();
let cloned = creator.create_tree().unwrap();
```

## clone_multibody

快速克隆函数。

```rust
let cloned = clone_multibody(&multibody).unwrap();
```

## SimpleTreeCreator

简单树创建器。

```rust
let creator = SimpleTreeCreator::new(5).unwrap();  // 5个物体
let multibody = creator.create_tree().unwrap();
```

## User2InternalIndex

用户索引到内部索引的映射。

```rust
let mut index_map = User2InternalIndex::new();

// 添加物体
index_map.add_body(0, -1);  // body_index, parent_index
index_map.add_body(1, 0);
index_map.add_body(2, 0);

// 构建映射
index_map.build_mapping();

// 转换索引
if let Some(internal) = index_map.user_to_internal(0) {
    println!("User 0 -> Internal {}", internal);
}

if let Some(user) = index_map.internal_to_user(0) {
    println!("Internal 0 -> User {}", user);
}
```

## Random 工具

逆动力学模块提供的随机数工具。

```rust
use nekobullet::id_utils::random;

// 初始化随机数生成器
random::init();
random::init_with_seed(12345);

// 生成随机数
let int_val = random::random_int(0, 100);
let float_val = random::random_float(0.0, 1.0);

// 生成随机物理参数
let mass = random::random_mass();
let (ix, iy, iz) = random::random_inertia_principal();
let (ax, ay, az) = random::random_axis();
```

## 完整示例

### 机械臂

```rust
use nekobullet::*;

fn main() {
    let mut multibody = MultiBody::new();

    // 基座（固定）
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

    // 第一关节（旋转）
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

    // 第二关节（旋转）
    multibody.add_body(
        2, 1,
        JointType::Revolute,
        Vec3::new(0.0, 1.0, 0.0),
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        Vec3::new(0.0, 1.0, 0.0),
        3.0,
        Vec3::new(0.0, 0.5, 0.0),
        [[0.05, 0.0, 0.0], [0.0, 0.05, 0.0], [0.0, 0.0, 0.05]],
    );

    // 完成
    multibody.finalize();
    multibody.set_gravity(Vec3::new(0.0, -9.81, 0.0));

    println!("MultiBody: {} bodies, {} DOFs", 
             multibody.num_bodies(), multibody.num_dofs());

    // 计算逆动力学
    let q = vec![0.0, 0.0];  // 关节位置
    let u = vec![0.0, 0.0];  // 关节速度
    let dot_u = vec![1.0, 1.0];  // 关节加速度
    let mut joint_forces = vec![0.0, 0.0];

    multibody.calculate_inverse_dynamics(&q, &u, &dot_u, &mut joint_forces);

    println!("Required joint forces: {:?}", joint_forces);
}
```

## MultiBody API

```rust
impl MultiBody {
    pub fn new() -> Self;
    pub fn handle(&self) -> *mut c_void;
    
    pub fn add_body(
        &mut self,
        body_index: i32,
        parent_index: i32,
        joint_type: JointType,
        parent_r: Vec3,
        body_t_parent: [[Real; 3]; 3],
        axis: Vec3,
        mass: Real,
        com: Vec3,
        inertia: [[Real; 3]; 3],
    ) -> i32;
    
    pub fn finalize(&mut self) -> i32;
    pub fn is_finalized(&self) -> bool;
    
    pub fn calculate_inverse_dynamics(
        &self,
        q: &[Real],
        u: &[Real],
        dot_u: &[Real],
        joint_forces: &mut [Real],
    ) -> i32;
    
    pub fn calculate_mass_matrix(
        &self,
        q: &[Real],
        mass_matrix: &mut [Real],
        initialize: bool,
        set_lower_triangular: bool,
    ) -> i32;
    
    pub fn num_bodies(&self) -> i32;
    pub fn num_dofs(&self) -> i32;
    pub fn set_gravity(&self, gravity: Vec3);
    pub fn set_accept_invalid_mass(&self, accept: bool);
    pub fn print_tree(&self);
}
```

## 性能提示

1. **合理设置自由度**：减少不必要的自由度
2. **使用正确的关节类型**：选择最适合的关节类型
3. **预处理**：对于固定的结构，可以预先计算一些参数
4. **数值稳定性**：注意惯性张量的设置
