fn main() {
    let x = 1;
    let y: &isize = &x;
    let mut a = 1;
    println!("{}", y);

    let b = &mut a;
    println!("{}", b);
    *b = 2;
    println!("{}", *b);

}
