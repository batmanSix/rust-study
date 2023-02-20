fn main() {
    //declare an array
    let a = [10, 20, 30];
    let mut iter = a.iter();
    // fetch an iterator object for the array
    println!("{:?}", iter);
    //fetch individual values from the iterator object
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    item_for();
    match_func();
    close_func();
}

fn item_for() {
    let a = [10, 20, 30];
    let iter = a.iter();
    for data in iter {
        print!("{}\t", data);
    }
}

fn match_func() {
    let names = vec!["Kannan", "Mohtashim", "Kiran"];
    for name in names.into_iter() {
        match name {
            "Mohtashim" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // cannot reuse the collection after iteration
    //println!("{:?}",names);
    //Error:Cannot access after ownership move
}

fn close_func() {
    let is_even = |x| x % 2 == 0;
    let no = 14;
    println!("{} is even ? {}", no, is_even(no));

    let val = 10;
    // declared outside
    let closure2 = |x| {
        x + val //inner function accessing outer fn variable
    };
    println!("{}", closure2(4));
}
