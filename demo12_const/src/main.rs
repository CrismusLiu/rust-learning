//! Rust 自定义类型：常量

// const类型值不可变
const CAPACITY: i32 = 10;
// static修饰的变量默认值不可变
static LANGUAGE: &'static str = "Rust";
// 可变的static变量值可被修改
static mut NAME: &'static str = "zhangsan";

fn main() {
    // 错误！const变量值不可修改
    // CAPACITY = 11;
    unsafe {
        NAME = "java";
        println!("NAME={}", NAME);
    }

    println!("CAPACITY={} LANGUAGE={}", CAPACITY, LANGUAGE);

    println!("判断是否合法：{}", is_valid(5));
}

fn is_valid(len: i32) -> bool {
    len - CAPACITY < 0
}
