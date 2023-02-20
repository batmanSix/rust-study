use std::io::Read;
fn main() {
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    remove_func();
}

use std::fs;
fn remove_func() {
    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");
}
