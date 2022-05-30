//! Rust trait

use std::fmt;

fn main() {
    // 定义结构体 Post
    #[derive(Debug)]
    pub struct Post {
        pub title: String,
        pub author: String,
        pub content: String,
    }

    // 为结构体 Post 定义函数
    impl Post {
        pub fn test() {
            println!("test");
        }
    }

    // 定义父 trait
    pub trait ParentSummary {
        fn to_string(&self) -> String;
    }

    // 定义 trait Summary，继承 ParentSummary
    pub trait Summary: ParentSummary {
        fn summarize_author(&self) -> &String;
        // 默认方法
        fn summarize(&self) -> String {
            format!("default: {}", self.summarize_author())
        }

        fn summarize1(&self) -> String {
            format!("default: {}", self.summarize_author())
        }
    }

    // 为结构体 Post 实现 Summary trait
    impl Summary for Post {
        fn summarize_author(&self) -> &String {
            &self.author
        }
        // 覆盖trait默认方法
        fn summarize(&self) -> String {
            format!("custom: {}", self.summarize_author())
        }
    }

    // 同时Post必须实现父trait ParentSummary
    impl ParentSummary for Post {
        fn to_string(&self) -> String {
            format!(
                "作者：{} 标题：{} 内容：{}",
                self.author, self.title, self.content
            )
        }
    }

    // 初始化 Post 对象
    // let post: &dyn Summary = &Post {
    let post = Post {
        title: String::from("标题"),
        author: String::from("作者"),
        content: String::from("内容xxx"),
    };

    // 调用对象post中方法
    println!("调用对象post中方法summarize: {}", post.summarize());
    // 错误！post只能调用其impl的trait中的方法
    // post.test();
    // 调用结构体 Post 中定义的函数 test
    Post::test();

    // 特征 trait 作为函数参数
    pub fn notify(item: &impl Summary) {
        println!("trait作为函数参数：{}", item.summarize());
    }

    notify(&post);

    // 特征 trait 作为函数返回值
    fn returns_summarize() -> impl Summary {
        Post {
            title: String::from("题目"),
            author: String::from("张三"),
            content: String::from("hello world"),
        }
    }
    let post_ret = returns_summarize();

    println!("trait作为函数返回值: {:#?}", post_ret.to_string());

    // 泛型的特征约束
    pub fn notify_generic_first<T: Summary + ?Sized>(item: &T) {
        println!("泛型特征约束1 {}", item.summarize());
    }
    notify_generic_first(&post);

    // 泛型特征的多重约束
    pub fn notify_generic_second<T>(item: &T)
    where
        T: Summary + fmt::Display,
    {
        println!("泛型特征约束2 {}", item.summarize());
    }

    impl fmt::Display for Post {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}, {}, {}", self.author, self.title, self.content)
        }
    }

    // 报错：the trait `std::fmt::Display` is not implemented for `Post`
    // 必须给 Post 实现trait Display
    notify_generic_second(&post);

    // trait Object
    let p_obj: &dyn ParentSummary = &Post {
        title: String::from("题目1"),
        author: String::from("张三1"),
        content: String::from("hello world1"),
    };
    println!("trait Object：{}", p_obj.to_string());

    // 关联类型
    trait Container {
        type A;
        fn contains(&self, a: &Self::A) -> bool;
        fn get_author(&self) -> &Self::A;
    }

    impl Container for Post {
        type A = String;
        fn contains(&self, a: &Self::A) -> bool {
            self.author.contains(a)
        }
        fn get_author(&self) -> &Self::A {
            &self.author
        }
    }

    let pp: &dyn Container<A = String> = &Post {
        title: String::from("题目"),
        author: String::from("张三"),
        content: String::from("hello world"),
    };

    println!("作者中是否包含\"三\"：{}", pp.contains(&String::from("三")));

    println!("获取作者：{}", pp.get_author());
}
