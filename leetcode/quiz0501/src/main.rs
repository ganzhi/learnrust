use std::collections::HashMap;

struct WordFilter {
    dict:HashMap<String, i32>
}

fn gen_variants(w: &String) -> Vec<String> {
    let mut res = Vec::<String>::new();
    let len = w.len();
    for i in 0..len+1 {        
        let suffix = &w[len-i..len];
        for j in 0..len+1 {
            let prefix = &w[0..j];
            let k = format!("{}#{}", suffix, prefix);
            res.push(k)
        }
    }
    return res;
}

impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut dict = HashMap::<String, i32>::new();
        for (i,w) in words.iter().enumerate() {
            let i = i as i32;
            for v in gen_variants(w) {
                match dict.get(&v) {
                    Some(prev) => {
                        if *prev < i {
                            dict.insert(v.to_owned(), i);
                        }
                    }
                    None => {dict.insert(v.to_owned(), i);}
                }
            }
            
        }
        WordFilter{dict}
    }
    
    fn f(&self, prefix: String, suffix: String) -> i32 {
        let q = format!("{}#{}", suffix, prefix);
        if let Some(r) = self.dict.get(&q) {
            return *r;
        }
        return -1;
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(prefix, suffix);
 */

fn main() {
    let words = vec!["abc".to_string()];
    let obj = WordFilter::new(words.into());
    let ret_1: i32 = obj.f("a".to_string(), "c".to_string());
    println!("Found the item at index: {}", ret_1)
}
