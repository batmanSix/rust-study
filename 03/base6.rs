// The `derive` attribute automatically creates the implementation
// required to make this `enum` printable with `fmt::Debug`.
// 匹配和枚举数据类型
#[derive(Debug)]
enum GenderCategory {
    Name(String),
    Usr_ID(i32),
}
fn main() {
    let p1 = GenderCategory::Name(String::from("Mohtashim"));
    let p2 = GenderCategory::Usr_ID(100);
    println!("{:?}", p1);
    println!("{:?}", p2);
    match p2 {
        GenderCategory::Name(val) => {
            println!("{}", val);
        }
        GenderCategory::Usr_ID(val) => {
            println!("{}", val);
        }
    }
}
