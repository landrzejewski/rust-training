use std::cell::RefCell;
use std::rc::{Rc, Weak};

type SinglyLink = Option<Box<SinglyNode>>;

#[derive(Debug)]
struct SinglyNode {
    element: i32,
    next: SinglyLink,
}

#[derive(Debug)]
struct SinglyLinkedList {
    head: SinglyLink,
}

impl SinglyLinkedList {
    fn new() -> Self {
        SinglyLinkedList { head: None }
    }

    fn push(&mut self, element: i32) {
        let old_head = self.head.take();
        self.head = Some(Box::new(SinglyNode {
            element,
            next: old_head,
        }));
    }

    fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.element)
            }
            None => None,
        }
    }

    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.element);
            current = &node.next;
        }
        println!("None");
    }
}

fn singly_linked_list() {
    let mut list = SinglyLinkedList::new();
    list.push(10);
    list.push(20);
    list.push(30);
    list.push(40);

    print!("after push: ");
    list.print();

    let popped = list.pop();
    println!("popped: {popped:?}");

    print!("after pop: ");
    list.print();

    while let Some(val) = list.pop() {
        println!("draining: {val}");
    }
    print!("empty: ");
    list.print();

    println!("singly_linked_list section executed");
}

type DoublyLink = Option<Rc<RefCell<DoublyNode>>>;

#[derive(Debug)]
struct DoublyNode {
    element: i32,
    next: DoublyLink,
    prev: DoublyLink,
}

impl DoublyNode {
    fn new(element: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(DoublyNode {
            element,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Debug)]
struct DoublyLinkedList {
    head: DoublyLink,
    tail: DoublyLink,
}

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, element: i32) {
        let new_node = DoublyNode::new(element);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());

                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.head.is_none() {
            return None;
        }

        let removed_val = self.head.as_ref().unwrap().borrow().element;
        self.head
            .take()
            .map(|old_head| match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            });
        Some(removed_val)
    }

    fn print(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{} <-> ", node.borrow().element);
            current = node.borrow().next.clone();
        }
        println!("None");
    }
}

fn doubly_linked_list() {
    let mut list = DoublyLinkedList::new();
    list.push(10);
    list.push(20);
    list.push(30);
    list.push(40);

    print!("after push: ");
    list.print();

    let popped = list.pop();
    println!("popped: {popped:?}");

    print!("after pop: ");
    list.print();

    while let Some(val) = list.pop() {
        println!("draining: {val}");
    }
    print!("empty: ");
    list.print();

    println!("doubly_linked_list section executed");
}

#[derive(Debug)]
struct CycleNode {
    next: Option<Weak<RefCell<CycleNode>>>,
}

impl Drop for CycleNode {
    fn drop(&mut self) {
        println!("  dropping CycleNode");
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct TreeNode {
    value: i32,
    parent: RefCell<Weak<TreeNode>>,
    children: RefCell<Vec<Rc<TreeNode>>>,
}

fn reference_cycles() {
    println!("creating a -> b -> c -> a cycle with Weak:");
    let a = Rc::new(RefCell::new(CycleNode { next: None }));
    println!(
        "  a: strong={}, weak={}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );

    let b = Rc::new(RefCell::new(CycleNode {
        next: Some(Rc::downgrade(&a)),
    }));
    println!(
        "  after b: a strong={}, weak={}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!(
        "           b strong={}, weak={}",
        Rc::strong_count(&b),
        Rc::weak_count(&b)
    );

    let c = Rc::new(RefCell::new(CycleNode {
        next: Some(Rc::downgrade(&b)),
    }));

    a.borrow_mut().next = Some(Rc::downgrade(&c));

    println!(
        "  after cycle: a strong={}, weak={}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!(
        "               b strong={}, weak={}",
        Rc::strong_count(&b),
        Rc::weak_count(&b)
    );
    println!(
        "               c strong={}, weak={}",
        Rc::strong_count(&c),
        Rc::weak_count(&c)
    );

    println!("\nparent-child tree with Weak:");
    let leaf = Rc::new(TreeNode {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(TreeNode {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    if let Some(parent) = leaf.parent.borrow().upgrade() {
        println!("  leaf's parent value: {}", parent.value);
    }

    println!(
        "  branch strong={}, weak={}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );
    println!(
        "  leaf strong={}, weak={}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    println!("reference_cycles section executed");
}

pub fn run() {
    singly_linked_list();
    doubly_linked_list();
    reference_cycles();
}
