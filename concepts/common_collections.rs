
use std::collections::HashMap;

let v: Vec<i32> = Vec::new();

let v = vec![1,2,3];

let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);



let v = vec![5,6,6,78];

let forth: Option<&i32> = v.get(3);

match forth {
    Some(forth) => println!("The forth element is {forth}"),
    None => println!("There is no forth element."),
}

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Orange"),40);

let color = String::from("Blue");
let score = scores.get(&color).copied().unwrap_or(0);

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{scores:?}");