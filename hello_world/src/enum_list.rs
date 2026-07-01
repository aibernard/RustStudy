// 利用Enum的变体互斥性（要么是有数据的Cons，要么是代表终点的Nil），在编译期把空指针彻底扼杀在摇篮里。
enum List {
    // 元组结构体（元素+next指针）
    // Rust 编译器在编译期必须百分之百确定一个类型在栈（Stack）上的物理字节大小。
    // Box<T>是Rust的堆内存分配智能指针。
    Cons(u32, Box<List>),
    // 表明链表末尾
    Nil,
}

#[allow(dead_code)]
// Methods can be attached to an enum
impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self, elem:u32) -> List {
        // 它在堆（Heap）上开辟了一块 16 字节的新空间，把整个老链表的数据从栈上 Move 移动到了堆里，并返回一个指向这块堆内存的 8 字节指针。
        List::Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail.
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html 
        match *self {
            List::Cons(_, ref  tail) => 1 + tail.len(),
            List::Nil => 0
            }
    }
        // Return representation of the list as a (heap allocated) string

    fn stringify(&self) -> String {
            match *self {
                List::Cons(head, ref tail) => {
                    // `format!` is similar to `print!`, but returns a heap
                    // allocated string instead of printing to the console
                    format!("{}, {}", head, tail.stringify())
                },
                List::Nil => {
                    format!("Nil")
                },
            }
    }
}
