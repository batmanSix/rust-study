// 条件语句if 不带括号
fn main() {
    let a = 12;
    let b;
    if a > 0 {
        b = 1;
    } else if a < 0 {
        b = -1;
    } else {
        b = 0;
    }
    println!("b is {}", b);

    // 三元表达式
    let a1 = 3;
    let number = if a1 > 0 { 1 } else { -1 };
    println!("number 为 {}", number);
}
