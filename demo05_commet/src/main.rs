fn main() {
    

    /*
    1、代码注释：
    单行注释：//
    块注释：/* … */

    2、文档注释：
    单行文档注释：///
    文档块注释：/** ... */

    3、包和模块级别的注释：//! 和 /*! ... */

    */
    // 输出hello, world
    println!("Hello, world!"); //输出hello, world

    let res = add(1, 2);
    println!("1+2={}", res);

    let res = sub(1, 2);
    println!("1-2={}", res);



    let res = demo05_commet::calc::add(1, 11);
    println!("{}", res);

    let res = demo05_commet::calc::sub(21, 11);
    println!("{}", res);


}

/// add：两个参数相加
fn add(x: i32, y: i32) -> i32 {
    x + y
}

/**
 * sub：求 x 将去 y 的结果
 */
fn sub(x: i32, y: i32) -> i32 {
    x - y
}

