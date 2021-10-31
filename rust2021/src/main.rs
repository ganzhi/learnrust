fn main() {
    for e in [1,2,3] {
        println!("{}", e);
    }
    println!("Hello, world!");

    // Now closure only capture what it reads.
    let x = 10;
    let c = || {
        let _ = x; // no-op and no data is read. Thus x can be used later
    };
    c();

    let c = || match x {
        _ => println!("Hello World from closure!")
    };
    c();
}
