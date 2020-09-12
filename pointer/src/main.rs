fn main() {
    let x = 5;
    let y = &x;

    let mut z = 1;

    z = *y;

    println!("Hello, world {}!", y);
}
