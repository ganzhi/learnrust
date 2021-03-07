static NUMBERS: [char; 20] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];

pub fn to_string_base(base: u32, num: u32) -> String {
    // allocate a string with capacity of 25 for reducing memory operation
    let mut buffer = String::with_capacity(25);
    let mut p = num;
    while p > 0 {
        let rem = p % base;
        p = p /base;
        let chr = NUMBERS[rem as usize];
        buffer.push(chr);
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

pub fn palindrome(base: u32, value: u32) -> Result<String, &'static str> {
    if base > 20 {
        return Err("Base is must be less than 11")
    }

    let text = to_string_base(base, value);
    if detect_palindrome(&text) {
        return Ok(text);
    }
    return Err("Not a palindrome");
}

#[cfg(test)]
mod tests {
    use super::palindrome;
    use super::to_string_base;

    #[test]
    fn test_palindrome() {
        let result = palindrome(5, 100);
        match result {
            Ok(value) => {
                panic!();
            },
            Err(_msg) => {
                println!("{}", to_string_base(5, 100));
            }
        }
    }

    #[test]
    fn test_to_string_base(){
        let res = to_string_base(5, 100);
        assert_eq!(res, "400");
    }
}