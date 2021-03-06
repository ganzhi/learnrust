fn to_string_base(num: u32, base: u32) -> String {
    let mut buffer = "".to_owned();
    let mut p = num;
    while p > 0 {
        let rem = p % base;
        p = p /base;
        buffer.push_str(&rem.to_string());
    }
    
    let result = buffer.chars().rev().collect::<String>();
    return result;
}

fn detect_palindrome(text: &String) -> bool{
    let bytes = text.as_bytes();
    let mut b = 0;
    let mut e = bytes.len() - 1;
    while e>b {
        if bytes[b] != bytes[e] {
            return false;
        }

        b = b + 1;
        e = e - 1;
    }
    return true;
}

fn main() {
    let base = 4;

    for i in 1..100 {
        let text = to_string_base(i, base);
        if detect_palindrome(&text) {
            println!("{}", text);
        }
    }
    println!("Hello, world!");
}
