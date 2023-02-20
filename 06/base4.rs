/*
  std::fs::File	open()
  pub fn open<P: AsRef>(path: P) -> Result	open 静态方法可用于以只读模式打开文件。
  std::fs::File	create()
  pub fn create<P: AsRef>(path: P) -> Result	静态方法以只写模式打开文件。如果文件已经存在，则旧内容将被销毁。否则，将创建一个新文件。
  std::fs::remove_file	remove_file()
  pub fn remove_file<P: AsRef>(path: P) -> Result<()>	从文件系统中删除文件。无法保证立即删除该文件。
  std::fs::OpenOptions	append()
  pub fn append(&mut self, append: bool) -> &mut OpenOptions	设置文件追加模式的选项。
  std::io::Writes	write_all()
  fn write_all(&mut self, buf: &[u8]) -> Result<()>	尝试将整个缓冲区写入此写入。
  std::io::Read	read_to_string()
  fn read_to_string(&mut self, buf: &mut String) -> Result	读取此源中 EOF 之前的所有字节，并将它们附加到 buf。
*/

use std::io::Write;
fn main() {
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Hello World".as_bytes())
        .expect("write failed");
    file.write_all("\nJC2182".as_bytes()).expect("write failed");
    println!("data written to file");
}
