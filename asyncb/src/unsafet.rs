use std::mem::size_of;

use futures_util::future::join;

static mut hello:i32 = 32;

fn main() {
    pretty_env_logger::init();

    let mut s = [1, 2, 3, 4, 5];
    let p = &s as *const i32;

    unsafe {
        println!("{}", *p.offset(1));
        hello = 48;
    }
    println!("The global hello is {}", hello);
}
