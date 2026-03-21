
//what if a reference OUTLIVES the data it points to?
//That would be a dangling pointer -> a reference to freed memory.
//That's a serious bug in many languages. Rust prevents it entirely.


//  A lifetime is just Rust's way of tracking HOW LONG a reference
//  is valid. Most of the time Rust figures it out automatically
//  (called "lifetime elision"). But sometimes YOU need to tell Rust
//  how the lifetimes of different references relate to each other.

fn why_lifetimes_exist() {
    let x = 5;
    let r = &x; // r borrows x

    /*
    let dangling;
    {
    
        let short_lived = String::from("I disappear soon");
        dangling = &short_lived; //borrow short_lived
    } //shorted_lived is DROPPED
    
    //dangling points to freed memory
    */

    //Rust sees that 'dangling' would outlive 'short_lived' and
    //refuse to compile. this is lifetime checking in action

}

// When You Need to Write Lifetime Annotations
// Annotations are needed when a function takes multiple references
// AND returns one of them. Rust can't tell which input the
// output came from, so you have to say.
fn lifetime_annotations() {

    let text = String::from("hello"):
    let result = first_word(&text);

}

// Rust can figure out the lifetime itself — no annotation needed.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i]; // return slice up to the space
        }
    }
    &s[..] // no space found — whole string is one word
}

fn longer_of_two() {
    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("xyz");
        //both string1 and 2 are alive here, so this is fine
        result = longest(&string1, &string2)
    }
    //strign2 is droppedhere
    //we cant use result after this point beacuse it MIGHT have pointed to string2.
    // rust enforces this via the 'a annotation
}

// Translation: "I promise the returned reference is only valid
// as long as BOTH inputs are still alive."

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}


// If a struct HOLDS a reference, you must annotate it.
// This tells Rust: "this struct cannot outlive the data
// that reference points to."

fn example_4_lifetime_in_struct() {

 
    let novel = String::from("Call me Ishmael. Some years ago...");
 
    // We create an Excerpt that holds a reference INTO `novel`.
    // The struct cannot outlive `novel`.
    let first_sentence;
    {
        // Find the first sentence
        let i = novel.find('.').unwrap_or(novel.len());
        first_sentence = Excerpt { part: &novel[..i] };
    }
    // `novel` is still alive here, so this is fine:

}

// The annotation 'a means: "an Excerpt cannot outlive the
// string slice stored in `part`."
struct Excerpt<'a> {
    part: &'a str,
}

// When you add methods to a struct that has lifetime annotations,
// the impl block also needs them. Looks verbose but follows a pattern.
fn example_5_lifetime_in_impl() {

 
    let text = String::from("Rust is awesome!");
    let excerpt = Excerpt { part: &text };
 
}

// impl<'a> means: "implementing for Excerpt that has lifetime 'a"
impl<'a> Excerpt<'a> {
    // This method returns a &str. Rust's elision rules figure out
    // the lifetime automatically — no annotation needed on `level`.
    fn announce(&self, news: &str) -> &str {
        println!("Attention: {}!", news);
        self.part // returning a reference tied to self's lifetime
    }
}

// 'static means the reference lives for the ENTIRE program.
// String literals are always 'static — they're baked into the binary.
// Don't reach for 'static to "fix" lifetime errors — usually wrong.

fn example_6_static_lifetime() {

 
    // String literals have lifetime 'static automatically.
    let s: &'static str = "I live forever in the binary.";
    println!("{}", s);
 
    // You'll also see 'static in trait bounds like:
    //   fn store<T: 'static>(value: T) { ... }
    // This means T contains no short-lived references —
    // it's safe to keep around as long as needed.
    println!("'static = lives as long as the whole program");
}

// A realistic example: a function that searches text and
// returns a reference to a matching line.
fn example_7_putting_it_together() {

 
    let haystack = String::from("one fish\ntwo fish\nred fish\nblue fish");
    let keyword = String::from("red");
 
    // find_line returns a reference into `haystack`.
    // The lifetime annotation ensures the result doesn't
    // outlive the string we're searching.
    match find_line(&haystack, &keyword) {
        Some(line) => println!("Found line: '{}'", line),
        None       => println!("Not found"),
    }
 
    // `haystack` still alive and valid here 
    println!("Original text still usable: {} chars", haystack.len());
}

// 'a ties the output lifetime to `text` (not `keyword`).
// We're returning a slice OF text, so the result lives as
// long as text does — keyword's lifetime doesn't matter here.
fn find_line<'a>(text: &'a str, keyword: &str) -> Option<&'a str> {
    for line in text.lines() {
        if line.contains(keyword) {
            return Some(line); // this slice points into `text`
        }
    }
    None
}