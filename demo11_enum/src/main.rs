#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    // 可以给枚举创建实现
    impl Color {
        pub fn create_green() -> Self {
            Color::Green
        }
    }

    let red = Color::Red; // 枚举值获取
    println!("{:?}", red);

    println!("{:?}", Color::create_green());

    // let msg = Message::Move { x: 2, y: 2 };
    let msg = Message::Quit;

    // if let 表达式
    if let Message::Move { x: a, y: b } = msg {
        assert_eq!(a, b);
    } else {
        panic!("Message非Move");
    }

    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg)
    }

    // 经典Option
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);

    let x: Option<u32> = None;
    assert_eq!(x.is_none(), true);
}

fn show_message(msg: Message) {
    println!("{:?}", msg);
}
