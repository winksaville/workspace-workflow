use alib;

fn main() {
    println!("Hello, world! invoke alib::add");

    let sum = alib::add(1, 2);
    println!("sum = {sum}");
    assert!(sum == 3);
}
