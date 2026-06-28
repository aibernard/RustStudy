// src/types.rs
// 货物包装：元组结构体
// 训练：无字段名复合类型。元组内一次包含（货物ID: u32, 重量： f64）
pub struct CargoBundle(pub u32, pub f64);

// 货运飞船：标准结构体
// 训练点：带字段名的复合类型，内含具有堆内存所有权的String
#[derive(Debug)]
pub struct Spaceship {
    pub name: String,
    pub fuel: u32,
    pub is_active: bool,
}

// 星际事件：高级代数数据类型（Enum ADT）
// 训练点：枚举类型，包含不同的变体，每个变体可以包含不同类型的数据，体验Rust强悍的模式匹配底座
#[derive(Debug)]
pub enum SpaceEvent {
    // 变体A：遭遇黑洞，携带一个标量数据：引力波强度系数
    BlackHole(f32),
    // 变体B：新飞船入港，携带一个完整的、为退化的Spaceship结构体
    ShipArrival(Spaceship),
    // 变体C: 货物倾销，携带一个命名空间结构
    CargoDump { rate: f32, code: char },
}
