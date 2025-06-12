struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node { value, next: None }
    }

    fn append(&mut self, value: i32) {
        match self.next {
            None => self.next = Some(Box::new(Node::new(value))),
            Some(ref mut node) => node.append(value),
        }
    }

    fn display(&self) {
        println!("{} ", self.value);
        match self.next {
            None => println!("None"),
            Some(ref node) => node.display(),
        }
    }
}

pub fn run() {
    let mut node = Node::new(1);
    node.append(2);
    node.append(3);
    node.display();
}