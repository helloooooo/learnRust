fn main() {
    println!("Hello World");
    let x = 1 + 2;
    println!("{}", x); // => 3
    let mut y = x;
    y = 5;
    let z = y;
    let x: i32 = 1;
    println!("{}", x);
    let x: &str = "abc";
    println!("{}", x);
}
