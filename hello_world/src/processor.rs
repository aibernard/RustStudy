// src/processor.rs
use crate::types::SpaceEvent;

// 函数A：计算安全剩余载荷
// 需求：
// - 输入：总载荷total和已用载荷used
// - 逻辑内声明一个局部块表达式{},在块内部计算total-used
// 块表达式和整个函数的最后一行，不准写return，不准加分号
pub fn calculate_remaining(total: f64, used: f64) -> f64 {
    let ret = total - used;
    ret
}

// 函数B： 事件黑盒处理器
// 解构高级枚举，所有权彻底转移
/// 需求：
///   - 输入一个事件 `event`（所有权转移进此函数）
///   - 使用 `match` 匹配 `SpaceEvent`：
///     - 如果是 `BlackHole(intensity)` -> 打印引力
///     - 如果是 `ShipArrival(mut ship)` -> 
///       将飞船的 `fuel` 修改为 100，并打印飞船信息。
///   - 🔥【思考并验证】：此函数结束后，传入的 `event` 和里面的 `ship` 会发生什么？
pub fn process_event(event: SpaceEvent) {
    match event {
        SpaceEvent::BlackHole(intensity) => {
            println!("Black hole intensity: {}", intensity);
        }
        SpaceEvent::ShipArrival(mut ship) => {
            ship.fuel = modify_fuel_safely(ship.fuel, -50);
            let total_shield = analyze_total_shield(&ship.shield_sectors);
            println!("🚢 飞船 [{}] 入港成功！", ship.name);
            println!(" -> 反应堆状态代码：{}", ship.core_status);
            println!(" -> 饱和安全计算后的剩余燃料：{} 升", ship.fuel);
            println!(" -> 四象限总护盾能级：{} 兆瓦", total_shield);
        }
        SpaceEvent::CargoDump { rate, code } => {
            println!("Cargo dumped with 倾销率：{}%，安全授权码：{}", rate, code);
        }
    }
}

/// 函数 C：星际序列号解析器
/// 训练点：字符串切片 `&str` 的只读视图与临时生命周期
/// 需求：
///   - 接收一个只读的字符串切片 `serial: &str`（形如 "BATCH-4092-X"）
///   - 返回该序列号的中间四位数字部分（同样作为只读切片 `&str` 返回）
///   - 提示：使用 `&serial[6..10]` 类似的行为
pub fn extract_batch_code(serial: &str) -> Option<&str> {
    if serial.len() >= 10 {
        Some(&serial[6..10])
    } else {
        None
    }

}

/// 🎯 原始类型专项练习 1：安全燃料灌装（饱和增减运算）
/// 需求：
///   - 接收当前燃料 `current` (u32)，以及变动量 `change` (i32，可正可负)
///   - 严禁直接使用 `+` 或 `-`（会发生编译期符号不匹配或运行时溢出）
///   - 利用 `current.saturating_add_signed(change)` 优雅实现：
///     如果燃料扣到负数，自动截断在 0；如果加到溢出，自动截断在 u32::MAX。
pub fn modify_fuel_safely(current: u32, change: i32) -> u32 {
    current.saturating_add_signed(change)
}

/// 🎯 原始类型专项练习 2：高阶固定数组切片分析
/// 需求：
///   - 接收护盾固定数组的引用 `shields: &[f32; 4]`
///   - 通过原始 `for ... in` 循环遍历这个数组，计算总护盾值
///   - 最终返回总护盾值 (f32)
pub fn analyze_total_shield(shields: &[f32; 4]) -> f32 {
    let mut total = 0.0;
    for &shield in shields.iter() {
        total += shield;
    }
    total
}