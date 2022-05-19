fn main() {
    // 元组结构体
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let p = Point(1, 2, 3);
    println!("{} {} {}", p.0, p.1, p.2);

    // 为结构体加方法
    impl Point {
        fn sum(&self) -> i32 {
            self.0 + self.1 + self.2
        }

        fn create(x: i32, y: i32, z: i32) -> Self {
            Point(x, y, z)
        }
    }

    // 调用结构体对象方法
    let sum = p.sum();
    println!("三个值的和：{}", sum);

    // 调用结构体函数，创建新结构体p2
    let p2 = Point::create(10, 11, 12);
    println!("{:?}", p2);

    #[derive(Debug)]
    struct Student {
        name: String,
        age: u8,
        active: bool,
    }

    let mut zhangsan = Student {
        name: String::from("zhangsan"),
        age: 10,
        active: true,
    };

    println!("修改前学生：{:?}", zhangsan);

    // 获取结构体的元素值
    println!("学生姓名: {}", zhangsan.name);

    // 修改name字段
    zhangsan.name = String::from("zhangsan_new");
    println!("修改后学生：{:?}", zhangsan);

    let lisi = Student {
        name: String::from("lisi"),
        active: true,
        ..zhangsan
    };

    println!("{:?}", lisi);
}
