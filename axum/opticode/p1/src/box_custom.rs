#[derive(Debug)]
pub struct LinkedListNode {
    val: i32,
    next: Option<Box<LinkedListNode>>,
}

impl LinkedListNode {
    fn new(val: i32) -> LinkedListNode {
        LinkedListNode { val, next: None }
    }
}
fn to_list(vector:Vec<i32>) -> Option<Box<LinkedListNode>> {
	let mut cur = None;
	for &value in vector.iter().rev() {
		let mut new_node = LinkedListNode::new(value); 
		new_node.next = cur;
		cur = Some(Box::new(new_node));
	}
	cur
}
pub fn run_box() {
    
	let vector = vec![0,1,2,3,4,5];
	println!("{:?}", to_list(vector));
}
