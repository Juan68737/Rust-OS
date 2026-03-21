

fn basic_borrow() {
    let name = String::from("alice"); //name is the owner

    let borrowed = &name; //borrwoed is a reference, it points to name's data

}

fn multiple_immutable_refs() {
    let book = String::from("The Rust Programming Language");

    //They all work fine
    let reader1 = &book;
    let reader2 = &book;
    let reader3 = &book;
}

fn mutable_reference() {

    let mut greeting = String::from("hello");

    // the mut means that greetings can change

    {
        let changer = &mut greeting; //mutable borrow
        changer.push_str(", world!"); // we modify it through the reference
        //changer is dropped and mutable borrow ends
    }

    //now greeting is "hello, world!"
}


// ONLY ONE Mutable Reference at a time
//this prevents data races. Two writers at once = chaos
fn one_mut_ref_at_a_time() {
    let mut score = 10;

    let editor = &mut score;
    //let editor2 = &mut score; ERROR

    *editor += 5; //the * deferences so we can change the actual value

    //After editor1 is done, we can create a new mutable refernce 
    let editor2 = &mut score;
    *editor2 *= 2;

    //you cannot mix a mutable ref with immuntable refs at the same time
    let a = &score; //immutable borrow starts
    let b = &score; // anoter immutable borrow - fine
    // let c = &mut score; // ERROR; cant have &mut while & is active

}

fn refs_in_functions() {
    let sentence = String::from("the quick brown fox");

    //we pass a reference - ownership stays with sentence
    let length = calculate_length(&sentence);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mutable_ref_in_function() {
    let mut message = String::from("hello");

    add_exclamation(&mut message);

}

fn add_exclamation(s: &mut String) {
    s.push('!'); //modify the String thouhg th emutable reference

}

fn slices() {
    let sentence = String::from("Hello World");

    //Slices reference a range inside the String

    let hello = &sentence[0..5];
    let world = &sentence[6..11];

    println!("first word:  {}", hello); // "hello"
    println!("second word: {}", world); // "world"
    println!("full string: {}", sentence); // still valid! 

    // String literals like "hello" are ALREADY &str slices —
    // they point directly into the program's read-only memory.
    let literal: &str = "I am already a slice";

    let owned = String::from("flexible");
    print_it(&owned);   // &String -> coerced to &str 
    print_it("direct"); // &str literal 
}

fn print_it(s: &str) {
    println!("print_it got: {}", s);
}