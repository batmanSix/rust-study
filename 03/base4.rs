// 枚举
// The `derive` attribute automatically creates the implementation
// required to make this `enum` printable with `fmt::Debug`.
/*
 该示例声明了一个枚举 - GenderCategory，它具有男性和女性的变体
 该打印！宏显示枚举的值。编译器将抛出一个错误
 特征 std::fmt::Debug 没有为 GenderCategory 实现。属性#[derive(Debug)]用于抑制此错误。
*/
/*
 定义
 enum enum_name {
   variant1,
   variant2,
   variant3
 }
*/
#[derive(Debug)]
enum GenderCategory {
    Male,
    Female,
}
fn main() {
    let male = GenderCategory::Male;
    let female = GenderCategory::Female;
    println!("{:?}", male);
    println!("{:?}", female);
}
