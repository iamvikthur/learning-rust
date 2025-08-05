use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("🧠 Memory Management in Rust");

    // OWNERSHITP
    let s1 = String::from("Ownership Example");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    println!("Ownership Transfered: {}", s2);
    // println!("{}", s1); // ❌ Error: s1 has been moved

    // BORROWING
    let s3 = String::from("Borrowing Example");
    borrow_demo(&s3);
    println!("Afer being borrowed: {}", s3); // still accessible

    // MUTABLE BORROWING
    let mut s4 = String::from("Hello");
    mutate_borrowed_string(&mut s4);
    println!("After mutable borrow: {}", s4); // now modified

    // LIFETIMES
    let result;
    let a = String::from("abcd");
    {
        let b = String::from("xyz");
        result = longest(&a, &b); // b's lifetime is shorter than a's
        println!("Longest String: {}", result);
    }

    // BOX (HEAP ALLOCATION)
    let boxed_value = Box::new(5);
    println!("Boxed Value: {}", boxed_value);

    // REFERENCE COUNTING
    let rc_value = Rc::new(String::from("Shared"));
    let rc_clone = Rc::clone(&rc_value);
    println!("Reference Counted Values: {}", rc_value);
    println!("Reference Counted Clone: {}", rc_clone);

    // REF CELL (MUTABLE BORROWING WITH REFERENCE COUNTING)
    let cell = RefCell::new(100);
    *cell.borrow_mut() += 50; // Mutate the value inside RefCell
    println!("RefCell Value: {}", cell.borrow());
}

fn borrow_demo(borrowed_data: &String) {
    println!("Borrowed String: {}", borrowed_data);
}

fn mutate_borrowed_string(borrowed_data: &mut String) {
    borrowed_data.push_str(", World!");
}

fn longest<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
