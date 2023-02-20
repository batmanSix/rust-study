// 循环
fn main() {
    let mut number = 1;
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT");
    for_func();
    array_func();
}

fn for_func() {
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("值为 : {}", i);
    }
}

fn array_func() {
    let a = [1, 2, 3, 4, 5];
    // a 是一个长度为 5 的整型数组

    let b = ["January", "February", "March"];
    // b 是一个长度为 3 的字符串数组

    let c: [i32; 5] = [1, 2, 3, 4, 5];
    // c 是一个长度为 5 的 i32 数组

    let d = [3; 5];
    // 等同于 let d = [3, 3, 3, 3, 3];

    let first = a[0];
    let second = a[1];
    // 数组访问

    // a[0] = 123; // 错误：数组 a 不可变
    // let mut a = [1, 2, 3];
    // a[0] = 4; // 正确

    // 元祖可以包含不同的数据
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup.0 等于 500
    // tup.1 等于 6.4
    // tup.2 等于 1
    let (x, y, z) = tup;
    // y 等于 6.4
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
    println!("tup is {}", 1)
}
