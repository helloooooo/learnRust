fn main() {
    let x: i32 = 7;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, x);

}
fn divergers() -> ! {
    panic!("THis function never returns!");
}
