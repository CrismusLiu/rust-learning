fn main() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array 原值: {:?}", &array);

    // 切片 _slice_all 取数组中所有元素
    let _slice_all = &array;

    // 切片slice1：从数组array中取下标0/1/2，不包括下标3的元素，&是引用，不可以改变值
    let slice1 = &array[0..3];
    println!("slice1: {:?}", slice1);
    // slice1[0] = 10; // `slice1` is a `&` reference, so the data it refers to cannot be written

    // 胖指针
    raw_slice(slice1);

    // 可变的数组切片：需要加关键字 mut，&mut 可变引用
    let slice2 = &mut array[2..5]; // [3, 4, 5]
    println!("切片 slice2 修改前: {:?}", slice2);
    // 修改第二个元素值
    slice2[1] = 11; // [3, 11, 5]
    println!("切片 slice2 修改前，修改第二个元素值: {:?}", slice2);
    change(slice2); // [10, 11, 5]
    println!("切片 slice2 修改后: {:?}", slice2);

    // 数组累加
    let sum_num = sum(slice2); // 26
    println!("slice2 和: {}", sum_num);

    println!("数组 array 修改后: {:?}", array); // [1, 2, 10, 11, 5]

    println!("{:?}", return_array());
}

// 胖指针输出
fn raw_slice(arr: &[i32]) {
    unsafe {
        let (val1, val2): (usize, usize) = std::mem::transmute(arr);
        // :x，代表的是小写的16进制，0x
        println!("胖指针>> 地址：{:x}, 切片长度：{:x}", val1, val2);
    }
}

//help: the trait `Sized` is not implemented for `[i32]`
// label: doesn't have a size known at compile-time
// fn change1(slice: [i32]) {
//     println!("{:?}", slice);
// }

// 修改第一个元素值
// 入参使用可变数组切片
fn change(slice: &mut [i32]) {
    slice[0] = 10;
}

// 数组元素值累加
fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

fn return_array() -> [i32; 5] {
    [0; 5]
}
