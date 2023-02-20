// rust 标准输入输出
/*
 Rust 的输入和输出标准库功能围绕两个特征进行组织 -
 1
 Read
 实现 Read 的类型具有面向字节输入的方法。他们被称为Reader
 Stdin,File
 2
 Write
 实现 Write 的类型支持面向字节和 UTF-8 文本输出。他们被称为Writer。
 Stdout,File
*/

// fn main() {
//     let mut line = String::new();
//     println!("Enter your name :");
//     let b1 = std::io::stdin().read_line(&mut line).unwrap();
//     println!("Hello , {}", line);
//     println!("no of bytes read , {}", b1);
//     cmd_fun();
// }
//main.rs
fn main(){
  let cmd_line = std::env::args();
  println!("No of elements in arguments is :{}",cmd_line.len()); 
  //print total number of values passed
  for arg in cmd_line {
     println!("[{}]",arg); //print all values passed 
     as commandline arguments
  }
}