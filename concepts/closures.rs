// closures.rs

fn apply_to_5<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(5)
}

fn main() {
    // Basic closure
    let add_one = |x: i32| x + 1;
    println!("add_one(5) = {}", add_one(5));

    // Closure with multiple parameters
    let multiply = |a: i32, b: i32| a * b;
    println!("multiply(3, 4) = {}", multiply(3, 4));

    // Closure capturing outer variable by reference
    let num = 10;
    let print_num = || println!("Captured num = {}", num);
    print_num();

    // Closure passed into a function
    let result = apply_to_5(|x| x * 2);
    println!("apply_to_5(|x| x * 2) = {}", result);

    // Mutable closure
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("count = {}", count);
    };

    increment();
    increment();

    // move closure takes ownership
    let name = String::from("Jonathan");
    let consume_name = move || {
        println!("Owned name = {}", name);
    };
    consume_name();
}
