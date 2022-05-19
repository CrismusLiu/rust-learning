//! Rust中的函数与方法

fn main() {
    // 函数返回
    let res1 = add(1, 4);
    let res2 = add(6, 4);
    println!("res1= {}  res2= {}", res1, res2);

    let p1 = Point { x: 1.0, y: 4.0 };
    println!("p1.x={}", p1.get_x());

    let mut r1 = Rectangle {
        start: Point::origin(),
        end: Point::new(3.0, 1.0),
    };

    // let mut r1 = Rectangle::create(Point::origin(), Point::new(3.0, 1.0));

    println!("矩形 移动前：{:?}", &r1);

    println!("r1 矩形面积: {}", &r1.area());

    r1.move_to(2.0, 3.0);
    println!("矩形 移动后：{:?}", &r1);
}

// 两种返回方式
fn add(x: i32, y: i32) -> i32 {
    if (x > 5) {
        // 使用 return 返回结果
        return -1;
    } else {
        // 直接返回结果
        x + y
    }
}

// 坐标点：Point
#[derive(Debug)]
struct Point {
    x: f64, // x坐标
    y: f64, // y坐标
}

impl Point {
    // 关联函数：有参数
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    // 关联函数：无参数
    fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    // 方法：第一个参数为 self 或其变体 &self 、&mut self
    fn get_x(&self) -> f64 {
        self.x
    }
}

// 矩形：Rectangle
#[derive(Debug)]
struct Rectangle {
    start: Point, // 起始：左上角坐标
    end: Point,   // 结尾：右下角坐标
}

impl Rectangle {
    // area方法：求矩形面积
    fn area(&self) -> f64 {
        let Point {
            x: start_x,
            y: start_y,
        } = self.start;

        let Point { x: end_x, y: end_y } = self.end;
        ((end_x - start_x) * (end_y - start_y)).abs()
    }

    // 矩形移动：start和end坐标分别移动x和y
    fn move_to(&mut self, x: f64, y: f64) {
        self.start.x += x;
        self.start.y += y;

        self.end.x += x;
        self.end.y += y;
    }

    fn create(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}
