
fn hello(_t: (i32,i32,i32)) {

}
fn main() {
    // tuple is "Copy" is all elements are Copy
    let x = (1,2,3);
    hello(x);
    println!("Hello, world! {}", x.0);

    let s = String::from("你好hello");

    let slice = &s[0..5];
    println!("{}", slice);
}
