//! Rust条件表达式

fn main() {
    let condition_val = 10;

    let res = condition_fn_enum(condition_val);
    // if let 表达式
    if let Some(data) = res {
        println!("data = {}", data);
    } else {
        println!("没有值！");
    }

    let res1 = condition_fn_val(1);
    // if let 表达式
    if let -1 = res1 {
        println!("data={}", res1);
    } else {
        println!("匹配失败！");
    }
}

fn condition_fn_enum(condition_val: i32) -> Option<i32> {
    // if 表达式
    if condition_val == 10 {
        Some(condition_val)
    } else {
        None
    }
}

fn condition_fn_val(condition_val: i32) -> i32 {
    // if 表达式
    if condition_val == 10 {
        condition_val
    } else {
        -1
    }
}
