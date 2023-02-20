fn main() {
    for i in 1..10 {
        for j in 1..i + 1 {
            print!("{} x {} = {}\t", j, i, j * i);
        }
        println!();
    }
}
