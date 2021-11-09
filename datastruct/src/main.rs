struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

fn main() {
    let child1 = Node{value:2, left:None, right: None};
    let child2 = Node{value:1, left:None, right: None};
    let parent = Node {value:0, left:Some(Box::new(child1)), right:Some(Box::new(child2))};
    print!("The value of parent is {}\n", parent.value);
    if let Some(c) = parent.left {
        print!("The value of left child is {}\n", c.value);
    }
    if let Some(c) = parent.right {
        print!("The value of right child is {} \n", c.value);
    }
}
