use std::rc::Rc;

pub fn run() {
    memory_management();
    //  lifetimes();
    // pointers();
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn memory_management() {
    /*
    Stack variables/memory
    - fast allocation and access
    - memory is automatically released/recovered when the scope in which the variable was declared ends
    - used when the size of a variable is known at compile time and cannot be changed (scalar types, arrays and tuples)
    - assigning to a new variable copies the value of the original variable
    */

    let mut a = 5;
    let b = a; // copy
    a = 10;
    {
        let c = 10;
    }
    //println!("{}, {}, {}", a, b, c);

    /*
       Heap variables/memory
       - high flexibility - memory can be dynamically allocated/allocated as needed (Vector, HashMap, String...) at the expense of allocation and access speed
       - variable may exist outside the range in which it was created (we can pass a reference to other part of the program)
       - memory is released automatically when the last owner ceases to exist
       - assignment to a new variable results in a change of ownership
    */

    /*
    let text = String::from("rust");
    let other_text = text; // transfer of ownership
    println!("{}", text);
    */

    /*
    let text = String::from("rust");
    let other_text = text.clone(); // cloning
    println!("{}", text);
    */

    /*
    let text = String::from("rust");
    show(text); // transfer of ownership
    println!("{}", text);
    */

    /*
    let text = String::from("rust");
    let text = show_with_result(text); // transfer of ownership x 2
    println!("{}", text);
    */

    /*
    let text = String::from("rust");
    show_with_ref(&text); // borrowing
    println!("{}", text);
    */

    let mut text = String::from("rust");
    show_with_mut_ref(&mut text); // borrowing
    println!("{}", text);

    let z = Box::new(3);

    /*
    In the case of complex types, allocated on the heap, we use a pointer (held on the stack),
    so it is possible to share the value of a variable without copying it. Memory deallocation takes place
    automatically when the owner's frame is destroyed. It is possible to make a copy of a complex type
    if the Clone trait is implemented (one have to call the clone method), in many cases this is inefficient,
    so it is better to use references (borrowing)
    */

    /*
    Ownership
    - provides security (no unpredictable behavior) for programs written in Rust https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    - Rust does not allow manual management of heap memory - this happens automatically (compile level, no garbage collector)
    - Rules:
      - all data in the heap can have only one owner
      - when the owner is no longer available (the end of the range in which it was defined), Rust releases the associated memory on the heap
      - memory ownership can be transferred when assigning to another variable or calling a function
      - memory on the heap can only be accessed by the current owner/holder
    */

    let string = String::from("text"); // always on the heap, dynamically allocated
    let str: &str = "Text"; // pointer to embedded memory, non-mutable

    let a = String::from("aaa");
    let b = &a;
    let c = &a;
    println!("{},{},{}", a, b, c); // is ok, the owner is one, for this we have 2 views read only

    let mut a = String::from("aaa");
    let b = &mut a;
    b.push('b'); // b is not the owner, it is just a read only reference
                 // let c = &a; // read/access is not possible because there is still a chance of mutation (line below)
    b.push('c'); // error

    /*
    Rust only allows 1 memory owner
    Rust allows for multiple references (many immutable or one mutable)
    see other examples below for more
    */
}

fn show(text: String) {
    println!("{text}");
}

fn show_with_result(text: String) -> String {
    println!("{text}");
    text
}

fn show_with_ref(text: &String) {
    println!("{text}");
}

fn show_with_mut_ref(text: &mut String) {
    text.push('!');
    println!("{text}");
}

#[derive(Debug, Clone, Copy)]
struct Test {
    value: i64,
}

fn mutate_test(mut test: Test) {
    test.value = 5;
    println!("{:?}", test);
}

// other examples

fn defining_many_immutable_references() {
    let s = String::from("hello");
    // you can define any number of immutable borrows.
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
}

fn restrictions_after_defining_mutable_reference() {
    let mut s = String::from("John");
    s.push_str(" Doe");
    // if you have a mutable borrow...
    let r1 = &mut s;
    // then you can't have any other borrows at all.
    // let r2 = &mut s;       // Nope!
    // let r3 = &s;           // Nope!
    // and you can't do this either (println! tries to borrow s).
    // println!("s: {}", s);  // Nope!
    r1.push_str("!");
    println!("r1: {}", r1);
}

fn restrictions_after_defining_immutable_reference() {
    let mut s = String::from("John");
    s.push_str(" Doe");
    // if you have immutable borrow(s)...
    let r1 = &s;
    let r2 = &s;
    // then you can't have any immutable borrows.
    // let r3 = &mut s;       // Nope!
    // and you also can't modify the original object either.
    // s.push_str(" dewey");  // Nope!
    println!("s: {}, r1: {}, r2: {}", s, r1, r2);
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn lifetimes() {
    /*
       - lifetime specifier does not change the lifetime of a reference,
         it only describes the relationship between the lifetimes of multiple references guaranteeing security
       - lifetime ensure that each reference refers to a valid / existing memory,
         in other words, it guarantees that the memory will not be cleared until it needs to be used/accessed
    */

    let x;
    {
        let y = 10;
        x = &y;
    }
    // println!("x: {x}"); // error - attempt to use a variable that no longer exists

    let a;
    {
        let b = String::from("text");
        a = &b;
    }
    // println!("{}", a); // error variable owner does not exist

    let s1 = String::from("abc");
    let result;
    {
        //let s2 = String::from("def");
        // result = get_longer(&s1, &s2);
        let s2 = "def"; // str is alive/exists for the duration of the application
        result = get_longer(s1.as_str(), s2);
    }
    println!("{}", result);

    let last_name = String::from("Kowalski");
    let address = String::from("tests");

    /*let client = Person {
        first_name: "Jan",
        last_name: &last_name,
        address,
    };*/
}

/*
 - for methods that have exactly one argument, the compiler assigns the same lifetime parameter to the argument as well as the result
 - for methods with arguments, the compiler assigns different (consecutive) lifetime parameters to the arguments and result
 - for methods with arguments that contain &self or &mut self lifetime of the result is the same as for the attribute &self or &mut self
*/

fn get_longer<'a>(text: &'a str, other_text: &'a str) -> &'a str {
    // in this case, the returned reference must be valid as long as the references of the passed arguments
    if text.len() >= other_text.len() {
        text
    } else {
        other_text
    }
}

// Person instance cannot survive longer than the properties it holds/stores
struct Person<'a, T> {
    first_name: &'a str,     // implicit static
    last_name: &'static str, // static denotes the lifetime of the entire program
    address: T,
}

// memory will be destroyed before potential use
// fn get_ref() -> &i32 {
//     let a = 4;
//     &a
// }

// is ok - ranges/scopes/lifetime of input and output are identical
fn get_ref(aa: &i32) -> &i32 {
    aa
}

// same with configuration
fn get_ref2<'a>(aa: &'a i32) -> &'a i32 {
    aa
}

fn pointers() {
    // Box<T> object - a simple smart pointer that points to data on the heap, the Box object itself lives on the stack
    let boxed_number = Box::new(42);

    // println!() can dereference the Box explicitly or implicitly
    println!("Explicitly dereferenced value: {}", *boxed_number);
    println!("Implicitly dereferenced value: {}", boxed_number);

    // if you want to use the value yourself, you must dereference explicitly
    let _value = *boxed_number;

    let a = Rc::new(Employee {
        name: String::from("Emily"),
        salary: 1000,
    });
    println!("Reference count initially is {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("Reference count after clone is {}", Rc::strong_count(&b));

    use_employee(&a);
    println!("Reference count after function is {}", Rc::strong_count(&a));

    if true {
        let d = Rc::clone(&a);
        println!("Reference count inside block is {}", Rc::strong_count(&d));
    }

    println!("Reference count after block is {}", Rc::strong_count(&a));

    // Note, Rust also supports non-owning "weak ref counts" which can be upgraded to "strong ref counts".
    // For details, see https://doc.rust-lang.org/std/rc/struct.Weak.html.
}

struct Employee {
    name: String,
    salary: u64,
}

fn use_employee(rc_emp: &Rc<Employee>) {
    let c = Rc::clone(&rc_emp);
    println!("Reference count inside function is {}", Rc::strong_count(&c));
}
