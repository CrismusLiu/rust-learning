fn main() {
    // 1、变量的绑定（所有权）
    let mut a: i32 = 10;
    println!("{}", a);
    a = 11;
    println!("{}", a);

    // 2、变量的可变性：mut

    // 3、变量的命名规范
    let _max_width = 10;

    let (x, _y) = (4, 5);
    println!("({})", x);

    // 4、变量与常量区别
    const MAX_WIDTH: u32 = 10_000_00;
    println!("{}", MAX_WIDTH);

    // 5、变量的shadowing特性
    let a = 10;
    let a = 100;
    let a = "1000";

    println!("{}", a);
}
