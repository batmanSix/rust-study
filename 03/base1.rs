// 定义变量
/*
  let xxx = 值
  let xxx : 数据类型 = 值 指定变量类型

  命名规范
  1 可以是字母数字下划线
  2 必须是字母下划线开头不能以数字开头
  3 变量名区分大小写
  4 mut关键字 mutable缩写 可变变量
*/
fn main() {
    let name: &str = "2131";
    let mut age: i32 = 15;
    age = 144;
    println!("{}", age);
    println!("{}", name);
}
