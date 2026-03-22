
fn basic_enum() {
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let heading = Direction::North;

    match heading {
        Direction::North => println!("Heading North"),
        Direction::South => println!("Heading South"),
        Direction::East  => println!("Heading East"),
        Direction::West  => println!("Heading West"),
    }
}

fn enum_with_data() {
    enum Fruit {
        Apple,
        Orange(u32),
        Banana(String),
    }

    let snack1 = Fruit::Apple;
    let snack2 = Fruit::Orange(3);
    let snack3 = Fruit::Banana(String::from("ripe"));

    match snack1 {
        Fruit::Apple          => println!("Just an apple"),
        Fruit::Orange(n)      => println!("Got {} oranges", n),
        Fruit::Banana(state)  => println!("A {} banana", state),
    }
 
    match snack2 {
        Fruit::Apple          => println!("Just an apple"),
        Fruit::Orange(n)      => println!("Got {} oranges", n),
        Fruit::Banana(state)  => println!("A {} banana", state),
    }
 
    match snack3 {
        Fruit::Apple          => println!("Just an apple"),
        Fruit::Orange(n)      => println!("Got {} oranges", n),
        Fruit::Banana(state)  => println!("A {} banana", state),
    }

}

fn different_data_per_variant() {

    enum Notification {
        Email { from: String, subject: String},
        Sms(String),
        Push {title: String, body: String},
        Silent
    }

    let not1 = Notification::Email {
        from: String::from("boss@work.com"),
        subject: String::from("Meeting at 3pm")
    };

    let not2 = Notification::Sms(String::from("Your code: 8842"));

    let not3 = Notification::Silent;

    print_notification(not1);
    print_notification(not2);
    print_notification(not3);
}

fn print_notification(n: Notification) {
    match n {
        Notification::Email { from, subject } => {
            println!(" Email from {}: {}", from, subject);
        }
        Notification::Sms(body) => {
            println!("SMS: {}", body);
        }
        Notification::Push { title, body } => {
            println!("Push — {}: {}", title, body);
        }
        Notification::Silent => {
            println!("(silent notification)");
        }
    }
}

fn methods_on_enum() {

    enum Season {
        Spring,
        Summer,
        Autumn,
        Winter,
    }

    impl Season {
        fn description(&self) -> &str {
            match self {
                Season::Spring => "warm and rainy",
                Season::Summer => "hot and sunny",
                Season::Autumn => "cool and windy",
                Season::Winter => "cold and snowy",
            }
        }

        fn is_warm(&self) -> bool {
            match self {
                Season::Spring | Season::Summer => true,
                _                               => false,
                // _ means "anything else" 
            }
        }
    }

    let now = Season::Autumn;
    println!("It is {}", now.description());
    println!("Is it warm? {}", now.is_warm());
 
    let summer = Season::Summer;
    println!("Summer: {} | warm: {}", summer.description(), summer.is_warm());
}

// Rust has no null. Instead it uses the built-in Option enum
// to represent "maybe a value, maybe nothing."
//
//   enum Option<T> {
//       Some(T),   // there IS a value, and here it is
//       None,      // there is NO value
//   }

fn option() {
    fn find_first_even(numbers: &[i32]) -> Option<i32> {
        for &n in numbers {
            if n % 2 == 0 {
                return Some(n); // found one — wrap it in Some
            }
        }
        None // found nothing
    }

    let num = vec![1,3,7,4,9];
    let result = find_first_even(&num);

    match result {
        Some(n) => println!("Found even number: {}", n),
        None    => println!("No even numbers found"),
    }

    let maybe: Option<i32> = Some(42);
 
    // .unwrap() — get the value, but PANIC if it's None (use carefully!)
    println!("unwrap: {}", maybe.unwrap());
 
    // .unwrap_or() — get the value, or use a default if None
    let empty: Option<i32> = None;
    println!("unwrap_or: {}", empty.unwrap_or(0));
}

// Result is another built-in enum for operations that can
// either SUCCEED or FAIL.
//
//   enum Result<T, E> {
//       Ok(T),   // success — carries the value
//       Err(E),  // failure — carries the error
//   }

fn result {
        // A function that can fail — dividing by zero is an error.
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("cannot divide by zero")) // failure
        } else {
            Ok(a / b) // success
        }
    }
 
    let good = divide(10.0, 2.0);
    let bad  = divide(5.0, 0.0);
 
    // Handle both outcomes.
    match good {
        Ok(value) => println!("10 / 2 = {}", value),
        Err(e)    => println!("Error: {}", e),
    }
 
    match bad {
        Ok(value) => println!("result = {}", value),
        Err(e)    => println!("Error: {}", e),
    }
 
    // Handy shortcuts (same idea as Option):
    // .unwrap()        — get Ok value or panic
    // .unwrap_or(val)  — get Ok value or use a default
    // .is_ok()         — returns true if Ok
    // .is_err()        — returns true if Err
    println!("is_ok:  {}", good.is_ok());  // true
    println!("is_err: {}", bad.is_err());  // true
}