fn main() {
    // 1、普通数组

    // 声明数组
    let mut a = [1, 2, 3, 4, 5];

    // [类型; 长度]
    // 声明数组b：值为5个1
    let mut _b = [1; 5];

    println!("数组a大小：{}", a.len());

    // 数组下标从0开始获取
    // 数组a中有5个元素，a[5]取的是第6个元素的值，
    // 运行报错：index out of bounds: the length is 5 but the index is 5
    // println!("{} {}", a[0], a[6]);

    println!("栈中分配 {} 个字节", std::mem::size_of_val(&a));

    // 修改数组中元素值
    _b[0] = 10;
    println!("b[0]= {}", _b[0]);

    //
    a = _b;
    println!("{:#?}", &a);
    println!("_b[0] = {}", &_b[0]);

    // 循环查询数组中元素值
    for i in a.iter() {
        println!("{}", i);
    }

    // 循环输出数组中元素值及其下标
    for item in a.iter().enumerate() {
        println!("下标：{} 值：{}", item.0, item.1);
    }

    //2、动态数组：Vec或vec!

    // Vec
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    println!("动态数组：Vec");
    for item in v1.iter().enumerate() {
        println!("下标：{} 值：{}", item.0, item.1);
    }

    v1[0] = 100;
    println!("修改后值：{}", v1[0]);
    v1.push(4);
    v1.push(5);
    for item in v1.iter().enumerate() {
        println!("下标：{} 值：{}", item.0, item.1);
    }

    // vec!
    let mut v2 = vec![1, 2, 3];

    println!("动态数组：vec!");
    for item in v2.iter().enumerate() {
        println!("下标：{} 值：{}", item.0, item.1);
    }

    v2[0] = 200;
    println!("修改后值：{}", v2[0]);

    // 删除指定下标的元素
    v2.remove(0);

    v2.push(4);
    v2.push(45);

    for item in v2.iter().enumerate() {
        println!("下标：{} 值：{}", item.0, item.1);
    }
}
