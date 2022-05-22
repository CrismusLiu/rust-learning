//! Rust 控制流-循环表达式：loop, while, for

fn main() {
    // 1、loop：无限循环，可通过break退出循环，并给外部一个返回值。

    let mut a = 0;
    loop {
        a += 1;
        print!("{}", a);
        if a == 20 {
            break;
        }
    }

    println!();

    let mut num = 0;
    let res = loop {
        num += 1;

        if num == 5 {
            break 100; // 循环中断，返回100
        }
    }; // 注意此处要有分号，因为这是一个变量的赋值语句

    println!("res={}", res);

    // 2、while：循环中无法在开头和结尾判断是否继续进行循环，而是在循环体中间某处控制循环的进行
    num = 0;
    while num != 4 {
        println!("num={}", num);
        num += 1;
    }
    println!("num最终结果={}", num);

    // 3、for：经常用来遍历线性数据结构，比如数组
    let a = [1, 2, 3, 4, 5];
    for i in 0..5 {
        if a[i] == 2 {
            println!("跳过：a[{}] = {}", i, 2);
            continue; // 不执行之后代码，继续执行下次循环
        }

        if a[i] == 5 {
            println!("中断：a[{}] = {}", i, a[i]);
            break; // 中断循环
        } else {
            println!("a[{}] = {}", i, a[i]);
        }
    }
}
