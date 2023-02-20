fn main() {
    // 浮点数
    //Rust 与其它语言一样支持 32 位浮点数（f32）和 64 位浮点数（f64）
    // 默认情况下，64.0 将表示 64 位浮点数
    // 因为现代计算机处理器对两种浮点数计算的速度几乎相同，但 64 位浮点数精度更高。
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x is {}", x);

    // char 字符
    // utf-8 作为底层编码 字符串是单引号
    let c: char = 'R';
    println!("c is {}", c);

    // boolean类型
    let t = true;

    let f: bool = false;

    println!("f is {}", f);
    println!("f is {}", t);
}
