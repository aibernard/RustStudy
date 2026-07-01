# Rust语言注释规则/语法详细介绍
Rust 提供了多种注释方式，分为普通注释和文档注释两大类。Rust 的注释语法设计得非常灵活，支持嵌套块注释，这是它与其他语言（如 C/C++）的重要区别。

## 普通注释（Non-doc Comments）
单行注释 `//`
```Rust
// 这是一行单行注释
fn main() {
    println!("Hello");  // 行尾注释
}
```
+ 从 // 开始直到行尾都被视为注释。
+ 常用于临时说明、调试信息或简短描述。

块注释 `/* */`
```Rust
/* 
   这是一个块注释，
   可以跨越多行。
*/

fn add(a: i32, b: i32) -> i32 {
    /* 临时屏蔽代码 
    let temp = a * 2;
    */
    a + b
}
```
重要特性：
Rust 支持块注释嵌套（这是 Rust 独有的便利设计）：
```Rust
/* 外层注释
   /* 内层注释 */
   外层继续...
*/
```
## 文档注释（Documentation Comments）
Rust 内置了强大的文档系统，文档注释会被rustdoc工具解析成HTML文档
外文档注释 `///`（最常用）
放在被文档化的项上方，用于函数、结构体、枚举、trait 等。
```Rust
/// 这是一个加法函数
/// 
/// # Examples
/// 
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
+ 支持 Markdown 语法（标题、代码块、列表、链接等）。
+ 常用标签：# Examples、# Panics、# Errors、# Safety 等。

内文档注释 `//!`
用于模块或 crate 本身的文档，通常放在文件顶部。
```Rust
//! # my_crate
//! 
//! 这是一个示例库，用于演示文档注释。

/// 这是模块内的公开函数
pub fn hello() {}
```
//! 注释的是当前模块，而非下面的条目。

# 指针类型
## 指针的基本分类
类型,语法,安全级别,大小,说明
引用,&T / &mut T,安全,通常 8 字节,最常用
原始指针,*const T / *mut T,不安全,通常 8 字节,Thin Pointer
胖指针 (Fat Pointer),&[T]、&str、&dyn Trait,安全（引用形式）,16 字节,重点
智能指针,Box<T>、Rc<T>、Arc<T>、String 等,安全,取决于实现,拥有所有权
## 胖指针
胖指针 = 数据指针 + 额外元数据（通常是 16 字节：8 字节指针 + 8 字节元数据）。
与普通指针（Thin Pointer）只存一个地址不同，胖指针携带了“描述自身”的额外信息。
常见的胖指针类型：
① 切片（Slice） - &[T] 和 &mut [T]
```Rust
let arr = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr[1..4];     // &[2, 3, 4]

// 胖指针内部结构（逻辑上）：
struct FatSlice {
    ptr: *const i32,   // 指向数据的指针
    len: usize,        // 长度（元数据）
}
```
包含：指针 + 长度
这就是为什么 &[T] 可以知道自己的长度，而裸指针 *const T 不知道。
② 字符串切片 - &str
```Rust
let s: &str = "hello world";

struct FatStr {
    ptr: *const u8,   // 指向 UTF-8 字节
    len: usize,       // 长度
}
```
本质也是胖指针（指针 + 长度）。
③ Trait Object（特征对象） - &dyn Trait
这是最典型的胖指针：
```Rust
trait Speak {
    fn speak(&self);
}

struct Dog;
impl Speak for Dog { fn speak(&self) { println!("汪"); } }

fn make_sound(animal: &dyn Speak) {  // 胖指针
    animal.speak();
}
// Trait Object 的胖指针结构：
struct TraitObject {
    data_ptr: *const (),      // 指向实际数据的指针
    vtable_ptr: *const (),    // 指向虚函数表 (vtable)
}
```
vtable（虚表）：存放该类型的方法指针列表，实现动态分发（类似 Java/C++ 的虚函数表）。
代码演示胖指针大小
```Rust
fn main() {
    println!("&i32 大小: {}", std::mem::size_of::<&i32>());           // 8
    println!("&[i32] 大小: {}", std::mem::size_of::<&[i32]>());       // 16
    println!("&str 大小: {}", std::mem::size_of::<&str>());           // 16
    println!("&dyn Speak 大小: {}", std::mem::size_of::<&dyn Speak>()); // 16
}
```
## 智能指针（Smart Pointers）
Box<T>：堆上分配的独占所有权指针（薄指针，内部是原始指针）。
Rc<T> / Arc<T>：引用计数指针（支持多所有权）。
String：胖指针（Vec<u8> 的封装，包含指针 + 长度 + 容量）。
Vec<T>：胖指针（指针 + 长度 + 容量）。

