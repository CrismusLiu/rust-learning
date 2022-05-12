fn main() {
    // debug模式编译时，Rust会检查整型溢出，会有panic
    // 当使用 --release 参数时，Rust不检测溢出，u8（0 ~ 2^n - 1） 最大值255，如果绑定值为 256 则变成 0，257 变成 1[高位舍弃]

    // 1、有符号整型：i8/i16/i32/i64/isize（指针宽度），值范围： -(2 ^ (n - 1)) ~ 2 ^ (n - 1) - 1
    let default_num = 1; // 默认的数值类型为：i32
    println!("{:?}", print_type_of(&default_num));

    // 方法pow是计算n次幂
    let vi8: i8 = 2_i8.pow(7) - 1; // -2^7 ~ 2^7 - 1
    let vi16: i16 = 2_i16.pow(15) - 1; // -2^15 ~ 2^15 - 1
    let vi32: i32 = 2_i32.pow(31) - 1; // -2^31 ~ 2^31 - 1
    let vi64: i64 = 2_i64.pow(63) - 1; // -2^63 ~ 2^63 - 1
    println!("{} {} {} {} {}", default_num, vi8, vi16, vi32, vi64);
    // 溢出校验，发送溢出返回None
    println!("{:?}", vi8.checked_add(0));
    println!("{:?}", vi8 + 2);

    // 2、无符号整型：u8/u16/u32/u64/usize（指针宽度），值范围：0 ~ 2^n - 1
    let vu8: u8 = 2_u8.pow(8) - 1; // 0 ~ 2^8 - 1
    let vu16: u16 = 2_u16.pow(16) - 1; // 0 ~ 2^16 - 1
    let vu32: u32 = 2_u32.pow(32) - 1; // 0 ~ 2^32 - 1
    let vu64: u64 = 2_u64.pow(64) - 1; // 0 ~ 2^64 - 1
    println!("{} {} {} {}", vu8, vu16, vu32, vu64);
    // 溢出校验
    println!("{:?}", vu8 + 1);

    // isize 和 usize 类型取决于程序运行的计算机 CPU 类型：
    //      若 CPU 是 32 位的，则这两个类型是 32 位的，同理，若 CPU 是 64 位，那么它们则是 64 位。

    // 3、浮点类型（带小数点的数字）：f32/f64，默认是f64
    let default_float = 1.0;
    let define_float = 1.0f32;
    let x: f32 = 1.0; // 值的长度，最大32位
    let y: f64 = 1.0; // 值的长度，最大64位，精度高于f32
    println!("{} {} {} {}", define_float, default_float, x, y);

    // 4、字符类型：char，如’a’, ‘b’，都是unicode字符，长度为4字节
    let a: char = 'a';
    let b = '中';
    let c = '🤣';
    println!("{} {} {}", a, b, c);

    // 5、布尔类型：bool，只能是true或false
    let default_flag = true;
    let flag2: bool = false; // 显式指定类型
    println!("{} {}", default_flag, flag2);

    // 6、单元类型：()，只有唯一的值()
    let tup = ();
    println!("{:?}", tup);
}

// 打印变量的类型
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
