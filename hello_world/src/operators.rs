#[derive(Debug, PartialEq)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

use std::ops::Add;
impl Add for Vector2D {
    type Output = Vector2D; // 关联类型：告诉编译器相加后产出什么类型
    /// 当编译器看到 let v3 = v1 + v2; 时，它在底层直接将其脱去糖衣（Desugaring），改写成了函数调用：
    /// ```
    /// let v3 = v1.add(v2); // 也可以写成 Vector2D::add(v1, v2);
    /// ``` 
    /// self 和 rhs 了吗？它们前面没有 & 符号！
    /// 这个函数要求绝对的传入值所有权
    // rhs 代表Right hand Side（等号右边的值）
    fn add(self, rhs: Vector2D) -> Vector2D {
        Vector2D{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    pub fn test_basic_arithmetic() {
        let a: i32 = 10_i32;
        let b: i32 = 20_i32;
        let c: i32 = 5_i32;
        assert_eq!(a + b * c, 110);
        let d = 2.5_f64;
        assert_eq!(a as f64 + d, 12.5);
    }

    #[test]
   pub fn test_bitwise_ops() {
       let read = 0b0000_0001_u8;
       let write = 0b0000_0010_u8;
       let execute = 0b0000_0100_u8;
       let my_permission = read | write;
       assert_eq!(my_permission & execute, 0);
       assert_eq!(read << 2, execute);
   }

   #[test]
   pub fn test_operator_overloading() {
       let v1 = Vector2D {x: 1.0, y: 2.0};
       let v2 = Vector2D {x: 3.0, y: 4.0};
       // 因为 Vector2D 没有实现 Copy 特质，所以 v1 的所有权被 Move 给了 self，v2 的所有权被 Move 给了 rhs。
       // 在 add 函数内部，利用这两个被吸进来的数据，拼装出了一个新的结构体并返回给了 v3。
       // 在 let v3 = v1 + v2; 这一行结束时，原先栈上的 v1 和 v2 槽位已经被彻底清空（未初始化状态）
       let v3 = v1 + v2;
    //    println!("{:?}", v3);
       assert_eq!(v3, Vector2D {x: 4.0, y: 6.0});
   }

   #[test]
   pub fn test_short_circuit() {
       assert!(true || (1 / 0 == 0));
   }
}