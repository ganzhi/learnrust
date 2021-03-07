mod palindrome;
fn main() {
    println!("Run \"cargo test\" to verify algorithm works");
    let base = 10;
    for i in 1..300 {
        let result = palindrome::palindrome(base, i*i);
        match result {
            Ok(text) => {
                println!("{} {}", i, text);
            },
            Err(_msg) => {

            }
        }
    }

    println!("{}", palindrome::to_string_base(5, 100));
}
