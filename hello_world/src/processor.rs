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
    // match 表达式的每个匹配分支通常以, 结尾，而不是分号。因为 match 是一个表达式，它会返回一个值(每个分支都应该返回相同类型的值)，而不是语句。
    // 普通表达式：pattern => expression,必须用逗号分隔。
    // 语句：pattern => { statement; statement; },必须用分号分隔。
    // 最后一个分支可以省略逗号，但为了保持一致性，通常建议在最后一个分支也加上逗号。
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
pub fn analyze_total_shield(shields: &[f32; 4]) -> f32 { //这里按引用传递，是出于性能考虑，避免拷贝整个数组
    let mut total = 0.0;
    // &shield 是解引用，获取数组元素的值，迭代器每一次弹出来（Yield）的元素，都是一个引用类型的值，所以我们需要解引用来获取实际的值
    for &shield in shields.iter() {
        total += shield;
    }
    total
}

pub fn mock_cargo_slice<R>(data: &[f32; 4], range: R) -> Vec<f32>
where
    R: std::ops::RangeBounds<usize>, //核心卡口：约束 R 必须实现了范围边界特质
{
    let mut result = Vec::new();

    let start = match range.start_bound() {
        std::ops::Bound::Included(&s) => {
            println!("start_bound Included, s {}", s);
            s
        },
        std::ops::Bound::Excluded(&s) => {
            println!("start_bound Included, s {}", s);
            s + 1
        },
        std::ops::Bound::Unbounded => 0, // 如果没有起点（如 ..5），默认从 0 开始
    };

    // range.start_bound()根本不返回数字！它返回的是一个包含三种可能性的“超级枚举（Enum）”——std::ops::Bound。
    //  # 为什么不直接返回数字？（数学多态性问题）
    // 1. 范围 A：1..5 （有起点，有终点）
    // 2. 范围 B：..5  （没有起点，左侧无限）
    // + 如果用户传入的是 1..5，你的 start_bound() 可以完美返回数字 1。
    // 但如果用户传入的是 ..5 呢？它压根就没有起点！这时候你的 start_bound() 应该返回什么数字？
    // 在 Java 里，你可能被迫返回一个魔术数字（比如 -1 或者 Integer.MIN_VALUE），或者返回 null。但这就埋下了臭名昭著的 NullPointerException 或逻辑越界漏洞。
    // Rust 坚决拒绝这种不安全的设计。为了同时兼容“有起点”、“没有起点”、“包含起点”等所有数学区间，Rust 让 start_bound() 统一返回标准库内置的 std::ops::Bound 枚举。
    // ```
    // pub enum Bound<T> {
    // Included(T), // 闭区间：边界包含这个值
    // Excluded(T), // 开区间：边界排除这个值
    // Unbounded,   // 无穷边界：这一侧压根没有值！
    // }
    // ```
    let end = match range.end_bound() {
        // match 模式匹配究竟匹配了什么？
        // match 的第一步，是去看这个枚举在内存里的 Tag（标签） 是什么。它是 Included？它是 Excluded？还是 Unbounded？
        // 一旦 Tag 匹配上了，match 就会执行 Rust 的独门绝技——解构（剥皮），把包裹在枚举内层的实际数值指针（Payload）活生生地抽出来，绑定给你指定的局部变量。
        std::ops::Bound::Included(&e) => {
            println!("end_bound Included, e {}", e);
            e + 1
        }, // 闭区间需要加 1 适配
        std::ops::Bound::Excluded(&e) => {
            println!("end_bound Excluded, e {}", e);
            e
        },
        std::ops::Bound::Unbounded => data.len(), // 如果没有终点（如 1..），默认到数组尽头
    };
    // 执行安全的内存切片提取
    for i in start..end {
        if i < data.len() {
            result.push(data[i]);
        }
    }
    result
}
// =================================================================
// 🔬 Range 高级特性自动化测试沙盒（追加到 src/processor.rs 底部）
// =================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;
    // 实验一：断言范围的”自消耗"与包含特性
    #[test]
    fn test_range_containment_and_iterator() {
        let mut r1 = 1..5; // [1, 5) 左闭右开区间]
        // 基础断言：利用原生的contains方法，进行物理边界检查
        assert!(r1.contains(&3));
        assert!(!r1.contains(&5));

        // 核心原理验证：范围在迭代时会”修改自身字段“
        // 本质是调用了Iterator::next(&mut self)
        assert_eq!(r1.next(), Some(1)); // 迭代器消耗了第一个元素
        assert_eq!(r1.next(), Some(2)); // 迭代器消耗了第二个元素，内部的start字段已经被修改为3
        assert!(!r1.contains(&1));

        // 高级链式调用，翻转范围
        // 注意：不能写 (1..5).rev() 的直接循环，因为 Range 本身没有实现 DoubleEndedIterator，
        // 需要通过 rev() 方法返回一个新的迭代器类型 Rev<Range>，所以必须先绑定到一个变量上
        let mut rev_range = (1..5).rev(); // 生成一个新的反向迭代器
        assert_eq!(rev_range.next(), Some(4)); // 反向迭代
        assert_eq!(rev_range.next(), Some(3));
    }

    // 测试Range在模式匹配中的解构能级
    #[test]
    fn test_range_pattern_matching() {
        let check_security_level = |alert_code: u32| -> &'static str {
            match alert_code {
                0..=10 => "SAFE",       // 全闭范围匹配：[0, 10]
                11..=50 => "WARNING",   // 全闭范围匹配：[11, 50]
                51..100 => "CRITICAL",  // 左闭右开范围匹配：[51, 100)
                _ => "UNKNOWN",
            }
        };
        assert_eq!(check_security_level(0), "SAFE");
        assert_eq!(check_security_level(10), "SAFE");
        assert_eq!(check_security_level(50), "WARNING");
        assert_eq!(check_security_level(99), "CRITICAL");
        assert_eq!(check_security_level(100), "UNKNOWN"); // 100 掉入了兜底分支
    }

    // 多态范围泛型RangeBounds<T>
    /// 在 Java 中，如果你想写一个接收各种不同边界的方法，你需要处理复杂的通配符（如 ? extends T）。
    /// 而在 Rust 的星际货运物流终端中，假设我们要编写一个“自动化货舱切割函数”，它不仅能接收 1..5，还要能接收 ..5（无起点）、1..（无终点）甚至 ..（全选）。
    /// 为了做到这一点，我们需要祭出 Rust 标准库的终极泛型特质：std::ops::RangeBounds<T>。
    #[test]
    fn test_universal_range_bounds() {
        let shields = [10.0, 20.0, 30.0, 40.0];

        // 1. 传入标准的左闭右开 Range (1..3) -> 期待提取索引 1, 2
        let res1 = mock_cargo_slice(&shields, 1..3);
        assert_eq!(res1, vec![20.0, 30.0]);

        // 2. 传入全闭 RangeInclusive (1..=3) -> 期待提取索引 1, 2, 3
        let res2 = mock_cargo_slice(&shields, 1..=3);
        assert_eq!(res2, vec![20.0, 30.0, 40.0]);

        // 3. 传入无起点 RangeTo (..2) -> 期待提取索引 0, 1
        let res3 = mock_cargo_slice(&shields, ..2);
        assert_eq!(res3, vec![10.0, 20.0]);

        // 4. 传入无终点 RangeFrom (2..) -> 期待提取索引 2, 3
        let res4 = mock_cargo_slice(&shields, 2..);
        assert_eq!(res4, vec![30.0, 40.0]);

        // 5. 传入零大小的全范围 RangeFull (..) -> 期待提取完整数组
        let res5 = mock_cargo_slice(&shields, ..);
        assert_eq!(res5, vec![10.0, 20.0, 30.0, 40.0]);
    }

    // 肉搏胖指针尺寸与隐式强制转换
    #[test]
    // coercion: 强迫、制止
    fn test_slice_fat_pointer_and_coercion() {
        // 物理尺寸断言，证明原始指针是8字节，而切片引用（胖指针）是16字节
        assert_eq!(size_of::<*const f32>(), 8);
        assert_eq!(size_of::<&[f32]>(), 16);
        assert_eq!(size_of::<&str>(), 16);

        // 解析胖指针的数据流向
        let array: [i32; 5] = [10, 20, 30, 40, 50];
        let slice: &[i32] = &array[1..4];// 截取20，30，40
        assert_eq!(slice.len(), 3);
        assert_eq!(slice[0], 20);

        // 高级特性：Deref 隐式强制转换
        // 在实际开发中，我们不需要到处写 Range。Vec 和 String 会自动退化为切片
        let vector_cargo: Vec<i32> = vec![1, 2, 3];
        let string_name: String = String::from("APOLLO");

        // 核心函数签名要求的是 &[i32] 和 &str
        fn read_dependencies(data: &[i32], name: &str) -> usize {
            data.len() + name.len()
        }
        // &Vec 自动转为 &[T]，&String 自动转为 &str
        // 这在底层是通过实现 std::ops::Deref 特质完成的，全程零拷贝、零运行时开销！
        let total_len = read_dependencies(&vector_cargo, &string_name);
        assert_eq!(total_len, 9);
    }

    // 高级滑动窗口（Windows）与数据分块（Chunks）的零拷贝解析
    #[test]
    fn test_slice_windows_and_chunks() {
        let telemetry_signals = [10.0, 12.5, 95.0, 88.0, 14.2];

        // 1. .windows(N) —— 连续重叠的滑动窗口（非常适合做游戏碰撞检测、AI 路径平滑、信号平滑）
        // 窗口大小为 2：生成 [10.0, 12.5], [12.5, 95.0], [95.0, 88.0] ...
        let mut window_iter = telemetry_signals.windows(2);
        let first_window = window_iter.next().unwrap();

        assert_eq!(first_window, &[10.0, 12.5]); // 提取出第一个窗口切片
        let second_window = window_iter.next().unwrap();
        assert_eq!(second_window, &[12.5, 95.0]);

        // 2. .chunks(N) —— 互不重叠的等长分块（极其适合多线程任务分发、批量渲染打包）
        // 块大小为 2：生成 [10.0, 12.5], [95.0, 88.0], [14.2]（最后一块不够就剩下多少给多少）
        let mut chunk_iter = telemetry_signals.chunks(2);
        assert_eq!(chunk_iter.next().unwrap(), &[10.0, 12.5]);
        assert_eq!(chunk_iter.next().unwrap(), &[95.0, 88.0]);
        assert_eq!(chunk_iter.next().unwrap(), &[14.2]); // 孤立的最后一块
        // 💡 架构师点评：上述所有的 windows 和 chunks 操作，在内存底层**没有发生过任何哪怕 1 字节的数据克隆**！
        // 它们仅仅是在栈上不停地生成新的 16 字节胖指针，指向原数组的不同区域，效率拉满！
    }

    // 传奇特技——可变切片的物理隔离安全拆分 (`split_at_mut`)
    #[test]
    fn test_mutable_slice_splitting() {
        let mut spaceship_bays = [100, 200, 300, 400]; // 四个货舱的物资量
        // 🚨 思考我们在所有权章节学的借用铁律：你不能同时借用两个可变引用指向同一个数组！
        // 如果你尝试直接写：
        // let bay0 = &mut spaceship_bays[0];
        // let bay1 = &mut spaceship_bays[1]; // 💥 编译器会直接无情击杀！报错：cannot borrow `spaceship_bays` as mutable more than once(second mutable borrow occurs here)
        // 为什么？因为编译器肉眼凡胎，它只知道你两次借用了同一个数组 `spaceship_bays` 的位置，
        // 它害怕这两个可变指针在运行时发生重叠（Aliasing），从而引发未定义行为。
        // println!("bay0 {}, bay1 {}", bay0, bay1);
        
        // 利用标准库的底层安全特技 `.split_at_mut(index)`
        // 它能把一个大的可变切片，在编译期物理切断成两个**类型完全独立、没有任何内存交集**的胖指针！
        let slice_mut: &mut [i32] = &mut spaceship_bays;
        let (left_zone, right_zone) = slice_mut.split_at_mut(2);
        // left_zone  控制原数组前 2 个元素：[100, 200] 的切片视图
        // right_zone 控制原数组后 2 个元素：[300, 400] 的切片视图
        left_zone[0] += 50;  // 修改货舱 0
        right_zone[0] += 50; // 修改货舱 2（即 right_zone 的第 0 个偏移）
    }

    
}