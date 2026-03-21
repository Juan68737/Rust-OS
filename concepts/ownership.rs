
fn basic_ownership() {
    println("Exmaple 1: Basic Ownership");
    let name = String::from("JhonaTech");
}

fn scope_and_drop() {
    let outer = String::from("outer_scope");

    {
        let inner = String::from("inner_scope");
        //inner drops here
    }

    //outer is dropped
}

fn move_semantics() {
    let s1 = String::from("hello");

    // Ownership of the String moves from s1 -> s2.
    // s1 is now INVALID. Rust will refuse to compile if you use it.
    let s2 = s1;

    // println!("{}", s1); //  ERROR: value moved — s1 no longer owns the data
    println!("s2 = {}", s2); // fine — s2 is the owner now

    // Why does Rust do this?
    // If both s1 and s2 tried to free the same memory when they go out
    // of scope, that would be a "double free" bug. Rust prevents it entirely
    // by making only ONE variable the owner.

}


// EXAMPLE 4 — Copy Types
// Simple types like integers, booleans, and chars are COPIED,
// not moved. They implement the `Copy` trait because they are
// small and live on the stack — cheap to duplicate.

fn copy_type() {

    let x = 5;
    let y = x; //x is copied into y, botgh are valid

    // Common Copy types: i32, u32, f64, bool, char, tuples of Copy types

    let flag = true;
    let other_flag = flag;

    // String is NOT Copy because it lives on the heap.
    // i32 IS Copy because it lives on the stack.

}


fn ownership_in_function() {
    let my_string = String::from("string");

    //ownership of string moves into takes_ownership
    takes_ownership(my_string);

    // println!("{}", my_string); //  ERROR: `my_string` was moved
    println(my_string);

    let x = 42;
    make_copy(x);

}

fn takes_ownership(some_string: String){
    //no some_string has owernship
}

fn make_copy(some_int: i32) {
    //some_int is a copy
}


fn return_ownership() {
    let s1 = String::from("string");

    // We pass s1 into the function AND get ownership back via the return.
    let s2 = gives_and_take_back(s1);
}

fn gives_and_take_back(some_strong: String) [
    some_string;
]

fn clone() {
    let s1 = String::from("string");

    // .clone() creates a full deep copy — s1 and s2 are independent.
    let s2 = s1.copy();

}