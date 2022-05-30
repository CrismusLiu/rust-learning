use core::marker::Copy;
use std::clone::Clone;
use std::fmt;

// 复合类型
#[derive(Debug, Clone, Copy)]
// #[derive(Debug)]
struct Student {
    age: i32,
}

// impl Drop for Student {
//     fn drop(&mut self) {
//         println!("释放对象：{:p}", self)
//     }
// }

fn main() {
    //作用域

    // 基本类型
    let num: i32 = 10;
    let num1 = num;

    let address = &num;
    let address1 = &num1;

    {
        let num2 = 12;
        println!("num2={}", num2);
    }
    // println!("{}", num2);

    println!(
        "num={}  num1={}",
        format!("{address:p}"),
        format!("{address1:p}")
    );

    let zhangsan = Student { age: 10 };
    // 释放资源函数：一个move语义的空函数
    // drop1(zhangsan);
    println!("{}", format!("{:p}", &zhangsan)); // 输出zhangsan地址

    test_student(zhangsan); // Copy语义

    {
        println!("zhangsan.age={}", zhangsan.age);
        println!("zhangsan 地址：{}", format!("{:p}", &zhangsan)); // 输出zhangsan地址
    }
    let stu = zhangsan; // 绑定新变量，Copy语义
    println!("{}", format!("{:p}", &stu)); // 输出stu地址
    println!("zhangsan.age={}", zhangsan.age);

    println!("main函数运行结束")
}

pub fn drop1<T>(_x: T) {
    println!("释放资源>>>>");
}

fn test_student(stu: Student) {
    println!("调用函数：test_student，stu地址：{}", format!("{:p}", &stu)); // 输出学生地址
    println!("{:#?}", &stu);
    println!(">>>>>>>>>>>>>>>>>>>>");
}
