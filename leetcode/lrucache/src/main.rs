struct LRUCache {

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        LRUCache {}
    }
    
    fn get(&self, key: i32) -> i32 {
        -1
    }
    
    fn put(&self, key: i32, value: i32) {
        
    }
}



fn main() {
    println!("Hello, world!");
    
 // Your LRUCache object will be instantiated and called as such:
    let obj = LRUCache::new(5);
    let ret_1: i32 = obj.get(1);
    obj.put(1, 2);
}
