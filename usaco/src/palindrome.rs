fn to_string_base(num: u32, base: u32) -> String {
    // allocate a string with capacity of 25 for reducing memory operation
    let mut buffer = String::with_capacity(25);
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

pub fn palindrome(base: u32, upper: u32) -> Result<Vec<String>, &'static str> {
    if base > 10 {
        return Err("Base is must be less than 11")
    }
    let mut result: Vec<String> = Vec::new();
    for i in 1..upper {
        let text = to_string_base(i, base);
        if detect_palindrome(&text) {
            result.push(text);
        }
    }
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::palindrome;

    #[test]
    fn test_palindrome() {
        let result = palindrome(5, 100);
        let res = result.unwrap();
        for l in &res {
            println!("{}", l);
        }
        assert_eq!(23, res.len());
        assert_eq!("343", res[22]);
    }
}