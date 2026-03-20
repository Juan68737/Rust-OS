fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // ownership moves from s1 to s2

    println!("{}", s2);

    // println!("{}", s1); // error: s1 no longer owns the value
}
