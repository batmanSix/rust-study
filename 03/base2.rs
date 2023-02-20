/*
  常量 const 定义常量
  常量名称通常是大写，常量不可以重复定义

  static 'static 可以是可变的变量
*/

fn main() {
    const PI: f64 = 3.1415926;
    println!("{}", PI);
    str_func();
}

/*
  字符串 &str std：str，字符串切片
  字符串对象
  String::new() 创建一个新的空字符串，他是静态的
  字符串在堆中分配
*/

fn str_func() {
    let s1: String = String::new();
    println!("s1:{},s1-len:{}", s1, s1.len());

    let s2: String = String::from("面向加薪学习");

    println!("s2:{},s2-len:{}", s2, s2.len());

    let mut s3: String = String::from("面向加薪学习");
    s3.push_str("我是康博"); // 追加
    println!("s2:{},s2-len:{}", s3, s3.len());

    let s4: String = String::from("面向加薪学习");
    // 替换字符串
    let res: String = s4.replace("面向加薪学习", "21313");
    println!("res is {}", res);
}
