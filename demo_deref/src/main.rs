//! Rust 解引用

use std::ops::Deref;

fn main() {
    // 定义结构体，其中只有一个成员变量
    struct DerefExample<T> {
        value: T,
    }

    // 为结构体 DerefExample 实现 Deref trait
    impl<T> Deref for DerefExample<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    let x = DerefExample { value: 'a' };
    // 使用 * 解引用结构体 DerefExample 中的成员变量
    assert_eq!('a', *x);
}
