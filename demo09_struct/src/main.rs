fn main() {
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

    println!("{:?}", zhangsan);

    // 获取结构体的元素值
    println!("student.name: {}", zhangsan.name);

    // 修改name字段
    zhangsan.name = String::from("zhangsan_new");
    println!("{:?}", zhangsan);

    let lisi = Student {
        name: String::from("lisi"),
        active: true,
        ..zhangsan
    };

    println!("{:?}", lisi);
}
