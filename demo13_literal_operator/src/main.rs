//! 字面量与运算符

fn main() {
    /* 字面量 */

    // 有后缀的字面量，它们的类型在初始化时就确定
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面量，它们的类型视使用情况而定
    let i = 1;
    let f = 1.0;

    // `size_of_val` 返回变量的大小，以字节（byte）为单位
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x)); // 1
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y)); // 4
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z)); // 4

    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i)); // 4
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f)); // 8

    /* 运算符 */

    let a1 = -10; // 整数值取负：-
    println!("a1={}", a1);

    let a2_1 = !10; // 整数值取反：!，-10 = !10+1
    let a2_2 = !false; // 布尔值取反：!
    println!("!10={}  !false={}", a2_1, a2_2);

    // 算数运算：+ - * / %
    let a3 = 10;
    let a3_1 = a3 + 2; // 12
    let a3_2 = a3 - 1; // 9
    let a3_3 = a3 * 3; // 30
    let a3_4 = a3 / 2; // 5
    let a3_5 = a3 % 2; // 0
    println!(
        "a3={} a3_1={} a3_2={} a3_3={} a3_4={} a3_5={}",
        a3, a3_1, a3_2, a3_3, a3_4, a3_5
    );

    // 位运算：& ^ ! << >>
    let a4_1 = 4 & 3; // 4 0100 3 0011 => 0000 = 0
    let a4_2 = 4 ^ 3; // 0111 => 7
    let a4_3 = !4; // 1111 ... 1011 => -4-1 = !4 => -5
    let a4_4 = 4 << 2; // 4*2*2 => 16
    let a4_5 = 4 >> 2; // 4/2/2 => 1
    println!(
        "a4_1={} a4_2={} a4_3={} a4_4={} a4_5={}",
        a4_1, a4_2, a4_3, a4_4, a4_5
    );

    // 位运算：& ^ ! ，布尔值运算
    let a4_6 = false & true; // false
    let a4_7 = true | false; // true
    let a4_8 = !true; // false
    println!("a4_6={} a4_7={} a4_8={}", a4_6, a4_7, a4_8);

    // 逻辑运算：& && | ||
    let a5_1 = false & true; // false
    let a5_2 = false && (10 != 11); // false
    let a5_3 = true || false; // true
    println!("a5_1={} a5_2={} a5_3={}", a5_1, a5_2, a5_3);

    // 复合运算：+= -= *= /= %= &= ^= <<= >>=
    let mut a6 = 1;
    a6 += 10; // 1+10= 11
    a6 -= 2; // 11-2= 9
    a6 *= 3; // 9*3= 27
    a6 /= 2; // 27/2 = 13
    println!("a6={}", a6);

    // 等值比较：== !=
    let a7_1 = 10;
    let a7_2 = 11;
    let res1 = a7_1 == a7_2; // false
    let res2 = a7_1 != a7_2; // true
    println!("res1={} res2={}", res1, res2);

    // 大小比较运算：< <= > >=
    let a8_1 = 10;
    let a8_2 = 11;
    println!(
        "{} {} {} {}",
        a8_1 > a8_2,  // false
        a8_1 >= a8_2, // false
        a8_1 < a8_2,  // true
        a8_1 <= a8_2  // true
    );
}
