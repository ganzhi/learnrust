struct Node<'a> {
    value: i32,
    left: Option<&'a Node<'a>>,
    right: Option<&'a Node<'a>>
}

fn main() {
    let child1 = Node{value:2, left:None, right: None};
    let child2 = Node{value:1, left:None, right: None};
    let parent = Node {value:0, left:Some(&child1), right:Some(&child2)};
    print!("The value of parent is {}\n", parent.value);
    if let Some(c) = parent.left {
        print!("The value of left child is {}\n", c.value);
    }
    if let Some(c) = parent.right {
        print!("The value of right child is {} \n", c.value);
    }    
}
