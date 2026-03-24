

//  A trait defines shared behavior. Like an interface in Java/Go, but better.
//  You say "anything that implements me must be able to do X."

trait Greet {
    fn hello(&self) -> String;
}

struct Human {
    name: String,
}
 
struct Alien {
    planet: String,
}

impl Greet for Human {
    fn hello(&self) -> String {
        format!("Hi, I'm {}!", self.name)
    }
}
 
impl Greet for Alien {
    fn hello(&self) -> String {
        format!("*telepathic noise from {}*", self.planet)
    }
}

fn basic_traits() {

    let h = Human {name: String::from("Alice")};
    let a = Alien { planet: String::from("Zorp") };
}

//  Traits can provide default implementations. Implementors can override them
//  or just use the freebie.

trait Describable {
    // Required — must implement this
    fn name(&self) -> &str;
 
    // Optional — has a default, override if you want
    fn describe(&self) -> String {
        format!("I am a thing called '{}'", self.name())
    }
 
    fn shout(&self) -> String {
        self.describe().to_uppercase()
    }
}
 
struct Rock;
struct Wizard { name: String }
 
impl Describable for Rock {
    fn name(&self) -> &str { "Rock" }
    // uses default describe() and shout()
}
 
impl Describable for Wizard {
    fn name(&self) -> &str { &self.name }
 
    // Override describe but keep default shout()
    fn describe(&self) -> String {
        format!(" I am the mighty wizard {}! ", self.name())
    }
}

fn default_methods() {
    let r = Rock;
    let w = Wizard {name: String::from("Gandalf")};
}

//  Rust can auto-implement many traits for you. Stop writing boilerplate.
//
//  Most useful derives:
//    Debug      — print with {:?} and {:#?}
//    Clone      — .clone() to copy a value
//    Copy       — implicit bitwise copy (only small types: ints, floats, etc.)
//    PartialEq  — == and !=
//    Eq         — full equality (use with PartialEq for HashMaps etc.)
//    PartialOrd — <, >, <=, >=
//    Ord        — full ordering (.sort() on Vec)
//    Hash       — use as HashMap/HashSet key
//    Default    — zero/empty default with T::default()


#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Point {
    x: f64,
    y: f64,
}
 
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Player {
    name: String,
    score: u32,
}
 
fn common_derives() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1.clone();
    let p3 = Point { x: 5.0, y: 6.0 };
 
    println!("{:?}", p1);            // Debug
    println!("p1 == p2: {}", p1 == p2); // PartialEq
    println!("p1 < p3: {}", p1 < p3);  // PartialOrd
 
    let default_player = Player::default(); // Default
    println!("{:?}", default_player); // Player { name: "", score: 0 }
 
    // Hash lets us use it as a HashMap key
    use std::collections::HashMap;
    let mut scores: HashMap<Player, String> = HashMap::new();
    let alice = Player { name: String::from("Alice"), score: 100 };
    scores.insert(alice.clone(), String::from("winner"));
}