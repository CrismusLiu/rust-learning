//! Rust 泛型

fn main() {
    // 1. 泛型示例
    {
        let arr1: [i32; 3] = [3, 2, 5];
        let largest = largest_i32(&arr1);
        println!("整型数组中最大值：{}", largest);

        let arr2: [char; 3] = ['c', 'b', 'a'];
        let largest = largest_char(&arr2);
        println!("字符数组中最大值：{}", largest);

        let largest = largest_generic(&arr1);
        println!("泛型>> 整型数组中最大值：{}", largest);

        let largest = largest_generic(&arr2);
        println!("泛型>> 字符数组中最大值：{}", largest);
    }

    // 2、结构体中使用泛型
    {
        // 一个泛型
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        let p1 = Point { x: 1, y: 3 };
        let p2 = Point { x: 1.0, y: 3.0 };
        let p3 = Point { x: 'b', y: 'a' };
        // x 与 y 类型必须一致
        // let p4 = Point {x: 1, y: 'a'};
        println!("struct p1={:?} p2={:?} p3={:?}", p1, p2, p3);

        // 多个泛型
        #[derive(Debug)]
        struct Student<I, T> {
            name: T,
            age: I,
        }
        // impl中使用泛型
        impl<I, T> Student<I, T> {
            fn get_name(&self) -> &T {
                &self.name
            }
        }

        let s1 = Student {
            name: String::from("zhangsan"),
            age: 10,
        };
        println!("struct s1={:?}", s1);

        let s2 = Student { name: 11, age: 10 };
        println!("struct s2={:?}", s2);

        // impl 方法使用
        println!("struct s1 name={}", s1.get_name());
    }

    // 3、枚举中使用泛型
    {
        #[derive(Debug)]
        enum Status<T, S> {
            SUCCESS(S, T),
            FAIL(S, T),
        }

        let s1 = Status::SUCCESS(200, "成功");
        let s2 = Status::FAIL(500, "失败");
        println!("enum s1={:?}", s1);
        println!("enum s2={:?}", s2);
    }

    // 4、impl中使用泛型

    // 5、使用trait泛型
    {
        #[derive(Debug)]
        struct Student<I, T> {
            name: T,
            age: I,
        }

        trait IStudent {
            fn get_name(&self) -> String;
        }

        impl IStudent for Student<i32, String> {
            fn get_name(&self) -> String {
                self.name.clone()
            }
        }

        // 泛型的trait约束
        fn test_trait_generic<T: IStudent>(stu: &T) {
            println!("test_trait_generic：{}", stu.get_name());
        }

        let s1 = Student {
            name: String::from("李四"),
            age: 10,
        };

        test_trait_generic(&s1);
    }

    // 6、const中使用泛型
    {
        // 打印数组
        fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
            println!("{:?}", arr);
        }

        let arr1 = [1, 2, 3, 4, 5];
        print_array(arr1);
    }
}

// 求整型数组中最大值
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 求字符数组中最大值
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 求数组中最大值
fn largest_generic<T: PartialOrd + Clone + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
