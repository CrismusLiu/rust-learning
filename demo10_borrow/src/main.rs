#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    complex_type();
    // common_type();
}

/// 复合数据类型
fn complex_type() {
    let mut s = String::from("hello");

    {
        let r1 = &s;
        let r2 = &s;

        // 报错！可变引用与不可变引用不能同时存在，因为如果后边程序使用到了不可变引用
        // let r3 = &mut s;
        // r3.push_str("world");

        println!("{}, {}", r1, r2); // 在定义了可变引用之后，再次访问不可变引用会报错
    }

    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_one = &point;
        let borrowed_two = &point;

        // 通过引用和原始所有者来访问数据
        println!(
            "Point : ({}, {}, {})",
            borrowed_one.x, borrowed_two.y, point.z
        );

        // 报错！不能可变地借用 `point` ，因为现在它有不可变的借用，如果后边程序使用到了不可变引用，其值无法确定
        // let mutable_borrow = &mut point;

        // 再次通过引用和原始所有者来访问数据
        println!(
            "Point : ({}, {}, {})",
            borrowed_one.x, borrowed_two.y, point.z
        )
    }

    // 再次可变地借用 `point`
    // let mutable_borrow = &mut point;

    {
        let mutable_borrow = &mut point;

        // 通过可变引用来改变数据
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // 报错！不能不可变地借用 `point`，因为现在它有可变的借用。
        //let y = &point.y;

        // 报错！不能打印，因为 `println!` 会创建一个不可变引用。
        // println!("Point Z coordinate is {}", point.z);

        // 可变引用可以作为不可变的传给 `println!`。
        println!(
            "Point : ({}, {}, {})",
            mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
        );
    }
    // 再次进行不可变借用
    let borrow_three = &point;
    println!("borrow_three {:#?}", borrow_three);
}

// 基本数据类型
fn common_type() {
    let mut var = 0_i32;
    println!("var地址：{}", format!("{:p}", &var));
    {
        let p1 = &mut var; // p1 指针本身不能被重新绑定,我们可以通过p1改变变量var的值
        *p1 = 1;
        println!("{}", var);
    }
    {
        let temp = 2_i32;
        println!("temp：{}", format!("{:p}", &temp));
        let mut p2 = &var; // 我们不能通过p2改变变量var的值,但p2指针本身指向的位置可以被改变
        println!("var：{} p2={}", format!("{:p}", &var), p2);
        println!("p2改前：{}", format!("{:p}", &p2));
        p2 = &temp;
        println!("p2改后：{}", format!("{:p}", &p2));
        println!("p2={} var={}", p2, var);
    }
    println!("var={}", var);
    {
        let mut temp = 3_i32;
        let mut p3 = &mut var; // 我们既可以通过p3改变变量var的值,而且p3指针本身指向的位置也可以改变
        *p3 = 3;

        println!("p3改前：{}", format!("{:p}", &p3));
        p3 = &mut temp;
        println!("p3改后：{}", format!("{:p}", &p3));
        println!("var={} p3={}", var, p3);
    }

    println!("var地址：{}", format!("{:p}", &var));
}
