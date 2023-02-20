// 结构体
struct Fruit {
    width: u32,
    height: u32,
}

impl Fruit {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

/*
 静态方法可以用作实用方法。这些方法甚至在结构被实例化之前就存在
 静态方法使用结构名称调用，无需实例即可访问。与普通方法不同，静态方法不会采用&self参数。
 语法 - 声明一个静态方法
 像函数和其他方法这样的静态方法可以选择包含参数。

 impl Structure_Name {

   fn method_name(param1: datatype, param2: datatype) -> return_type {

   }
}
*/

struct Point {
    x: i32,
    y: i32,
}
impl Point {
    //static method that creates objects of the Point structure
    fn getInstance(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
    //display values of the structure's field
    fn display(&self) {
        println!("x ={} y={}", self.x, self.y);
    }
}

fn main() {
    // 实例化结构体
    let small = Fruit {
        width: 10,
        height: 20,
    };

    println!("{},{},{}", small.width, small.height, small.area());

    let p1 = Point::getInstance(-10, 20);
    p1.display();
}
