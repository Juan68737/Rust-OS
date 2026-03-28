// Rust: Concise Control Flow with `if let` and `let ... else`
// -----------------------------------------------------------
// This file explains:
// 1. Why `if let` exists
// 2. How `if let` compares to `match`
// 3. How `let ... else` works
// 4. When to use each one
//
// Run mentally or copy into a Rust project to practice.

#[derive(Debug)]
enum Message {
    Quit,
    Move(i32, i32),
    Write(String),
}

fn example_match_with_option() {
    println!("\n--- example_match_with_option ---");

    let value = Some(5);

    // `match` is the full, explicit way.
    match value {
        Some(num) => println!("Got a value from match: {}", num),
        None => println!("Got nothing from match"),
    }
}

fn example_if_let_basic() {
    println!("\n--- example_if_let_basic ---");

    let value = Some(10);

    // `if let` means:
    // "If this value matches this pattern, run this block."
    if let Some(num) = value {
        println!("Got a value from if let: {}", num);
    }

    // If `value` had been `None`, this block would do nothing.
}

fn example_if_let_with_else() {
    println!("\n--- example_if_let_with_else ---");

    let value: Option<i32> = None;

    // `if let ... else` is useful when you care about one main case,
    // but still want to handle the other case.
    if let Some(num) = value {
        println!("Value exists: {}", num);
    } else {
        println!("Value was None");
    }
}

fn example_if_let_with_enum() {
    println!("\n--- example_if_let_with_enum ---");

    let msg = Message::Move(3, 7);

    // Here we only care about the Move variant.
    if let Message::Move(x, y) = msg {
        println!("Message is Move({}, {})", x, y);
    }
}

fn example_let_else_with_option(name: Option<&str>) {
    println!("\n--- example_let_else_with_option ---");

    // `let ... else` means:
    // "This pattern MUST match, or we leave early."
    let Some(actual_name) = name else {
        println!("No name was provided, so we return early.");
        return;
    };

    // If we get here, the pattern matched successfully.
    println!("Name is: {}", actual_name);
}

fn example_let_else_with_result(input: Result<i32, &str>) {
    println!("\n--- example_let_else_with_result ---");

    let Ok(number) = input else {
        println!("Could not read the number, so we return early.");
        return;
    };

    println!("Read number successfully: {}", number);
}

fn example_why_let_else_is_nice(user: Option<&str>) {
    println!("\n--- example_why_let_else_is_nice ---");

    // Without let-else, beginners often write nested code.
    // With let-else, we handle the bad case first and keep the happy path clean.
    let Some(name) = user else {
        println!("No user found.");
        return;
    };

    println!("Hello, {}!", name);
    println!("Now the rest of the function can use `name` normally.");
}

fn main() {
    example_match_with_option();
    example_if_let_basic();
    example_if_let_with_else();
    example_if_let_with_enum();

    example_let_else_with_option(Some("Jonathan"));
    example_let_else_with_option(None);

    example_let_else_with_result(Ok(42));
    example_let_else_with_result(Err("bad input"));

    example_why_let_else_is_nice(Some("Rust learner"));
    example_why_let_else_is_nice(None);
}

/*
QUICK SUMMARY
-------------

1. `match`
   Use when you want to handle ALL cases explicitly.

   Example:
       match value {
           Some(x) => println!("{}", x),
           None => println!("nothing"),
       }

2. `if let`
   Use when you only care about ONE pattern.

   Example:
       if let Some(x) = value {
           println!("{}", x);
       }

   Meaning:
   - If the pattern matches, run the block.
   - Otherwise, skip it.

3. `let ... else`
   Use when the pattern MUST match to continue.

   Example:
       let Some(x) = value else {
           return;
       };

   Meaning:
   - If the pattern matches, continue.
   - If it does not match, run the `else` block and exit early.

IMPORTANT RULE
--------------
The `else` block in `let ... else` must leave the current control flow.
That means it usually uses one of these:
- return
- break
- continue
- panic!

MENTAL MODEL
------------
- `if let` = "If this shape matches, do something."
- `let ... else` = "This shape must match, or stop right now."
*/
