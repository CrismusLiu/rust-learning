fn main() {
    // 元组a
    let mut a = (12, false);
    println!("元组a：{:?}", a);

    // 元组中只有一个值的情况
    let flag = (false,);
    println!("元组flag：{:?}", flag);

    let flag = (false);
    println!("flag值：{:?}", flag);

    // 基本数据类型：单元类型
    let _unit = ();
    println!("{:?}", _unit);

    // 访问数组元素
    println!("元组a中两个元素：{} {}", a.0, a.1);

    let (mut num, flag) = a;
    println!("元组a中两个元素：{} {}", num, flag);

    // 修改元组中某个元素的值
    num = 100;
    a.0 = 100;
    println!("修改后元组a中两个元素：{} {}", num, flag);
    println!("修改后元组a中两个元素：{:?}", a);

    let array = [1, 2, 3, 4, 5];
    let (first, length) = calc_length(&array);
    println!("第一个元素：{}  数组长度：{}", first, length);
}

fn calc_length(arr: &[i32]) -> (i32, usize) {
    (arr[0], arr.len())
}
