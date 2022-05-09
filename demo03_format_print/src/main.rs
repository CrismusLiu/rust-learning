use std::fmt;

fn main() {
    // printlnFn();
    // printFn();
    // formatFn();
    // eprintFn();
    // eprintlnFn();

    {
        let a = "abc";
        println!("{:p}", &a);
    }

    let a = "abc";
    println!("{:p}", &a);
}

fn printlnFn() {
    // println! 会检查使用到的参数数量是否正确。

    // 默认用法,打印Display
    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    println!("{} days", 31);

    // 在 `:` 后面指定特殊的格式
    println!("{:o}", 9); // 八进制
    println!("{:x}", 255); // 十六进制 小写
    println!("{:X}", 255); // 十六进制 大写
    println!("{:p}", &0); // 指针，最后有实例
    println!("{:b}", 15); // 二进制
    println!("{:e}", 10000f32); // 科学计数(小写)
    println!("{:E}", 10000f32); // 科学计数(大写)

    println!("{:?}", "test"); // Debug打印
    println!("{:#?}", ("test1", "test2")); // 带换行和缩进的Debug打印

    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用命名参数。
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 按指定宽度来右对齐文本。
    // 下面语句输出 " 1"，5 个空格后面连着 1。
    println!("{number:>width$}", number = 1, width = 6);

    // 在数字左边补 0。下面语句输出 "000001"。
    println!("{number:>0width$}", number = 1, width = 6);

    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure {
        x: i32,
        y: i32,
    }

    // impl fmt::Debug for Structure {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         f.debug_struct("Structure")
    //             .field("x", &self.x)
    //             .field("y", &self.y)
    //             .finish()
    //     }
    // }

    let s = Structure { x: 1, y: 2 };
    // println!("{:p}", &s);

    // 结构体这样的自定义类型需要更复杂的方式来处理。
    println!("This struct `{:#?}` won't print...", s);
}

fn printFn() {
    print!("{} days", 31);
    print!("{:o}", 9); // 八进制
    print!("{:x}", 255); // 十六进制 小写
}

fn print_str(out_str: &str) {
    println!("{}", out_str);
}

fn formatFn() {
    let mut str = format!("Hello"); // => "Hello"
    print_str(&str);

    str = format!("Hello, {}!", "world"); // => "Hello, world!"
    print_str(&str);

    str = format!("The number is {}", 1); // => "The number is 1"
    print_str(&str);

    str = format!("{:?}", (3, 4)); // => "(3, 4)"
    print_str(&str);

    str = format!("{value}", value = 4); // => "4"
    print_str(&str);

    let people = "Rustaceans";
    str = format!("Hello {people}!"); // => "Hello Rustaceans!"
    print_str(&str);

    str = format!("{} {}", 1, 2); // => "1 2"
    print_str(&str);

    str = format!("{:06}", 42); // => "000042" 长度不够6位的，前面补0
    print_str(&str);

    str = format!("{:#?}", (100, 200)); // => "(
                                        //       100,
                                        //       200,
                                        //     )"
    print_str(&str);
}

fn eprintFn() {
    eprint!("Error: Could not complete task1");
    eprint!("Error: Could not complete task2");
    eprint!("Error: Could not complete task3");
}

fn eprintlnFn() {
    eprintln!("Error: Could not complete task1");
    eprintln!("Error: Could not complete task2");
    eprintln!("Error: Could not complete task3");
}
