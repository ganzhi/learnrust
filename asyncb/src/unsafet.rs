static mut HELLO:i32 = 32;

fn main() {
    pretty_env_logger::init();

    let s = [1, 2, 3, 4, 5];
    let p = &s as *const i32;

    let s = unsafe {
        println!("{}", *p.offset(1));
        HELLO = 48;
        HELLO
    };
    println!("The global HELLO is {}", s);
}
