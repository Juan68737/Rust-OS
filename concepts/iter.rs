// iterators_explained.rs
// A beginner-friendly Rust file that explains iterators with examples.

fn main() {
    println!("=== Rust Iterators Explained ===\n");

    basic_iteration();
    iterator_methods();
    consuming_vs_non_consuming();
    map_filter_collect();
    enumerate_and_zip();
    custom_iterator_notes();
}

fn basic_iteration() {
    println!("1) Basic iteration");

    let nums = vec![10, 20, 30];

    // .iter() gives immutable references: &i32
    for n in nums.iter() {
        println!("Reading value by reference: {n}");
    }

    // nums is still usable because .iter() only borrowed it
    println!("nums is still here: {:?}\n", nums);
}

fn iterator_methods() {
    println!("2) Creating iterators in different ways");

    let nums = vec![1, 2, 3];

    // iter() -> immutable references
    let mut it1 = nums.iter();
    println!("iter(): {:?}", it1.next()); // Some(&1)
    println!("iter(): {:?}", it1.next()); // Some(&2)
    println!("iter(): {:?}", it1.next()); // Some(&3)
    println!("iter(): {:?}", it1.next()); // None

    let mut nums2 = vec![4, 5, 6];

    // iter_mut() -> mutable references
    for x in nums2.iter_mut() {
        *x += 100;
    }
    println!("iter_mut() changed the vector: {:?}", nums2);

    let nums3 = vec![7, 8, 9];

    // into_iter() takes ownership of the collection and yields owned values
    let mut it3 = nums3.into_iter();
    println!("into_iter(): {:?}", it3.next()); // Some(7)
    println!("into_iter(): {:?}", it3.next()); // Some(8)
    println!("into_iter(): {:?}", it3.next()); // Some(9)
    println!("into_iter(): {:?}\n", it3.next()); // None

    // nums3 cannot be used here anymore because ownership was moved
}

fn consuming_vs_non_consuming() {
    println!("3) Consuming vs non-consuming iterator methods");

    let nums = vec![1, 2, 3, 4];

    // Non-consuming adapter: map()
    // This creates a new iterator but does NOT actually do work yet.
    let doubled = nums.iter().map(|x| x * 2);

    // Nothing happens until we consume the iterator.
    println!("Iterators are lazy.");

    // Consuming adapter: collect()
    let doubled_vec: Vec<i32> = doubled.collect();
    println!("Collected doubled values: {:?}", doubled_vec);

    // Another consuming method: sum()
    let total: i32 = nums.iter().sum();
    println!("Sum of nums: {}\n", total);
}

fn map_filter_collect() {
    println!("4) Common chain: map + filter + collect");

    let nums = vec![1, 2, 3, 4, 5, 6];

    let result: Vec<i32> = nums
        .iter()                 // &i32
        .copied()               // i32 (copies because i32 is Copy)
        .filter(|x| x % 2 == 0) // keep even numbers
        .map(|x| x * 10)        // transform them
        .collect();             // gather into Vec<i32>

    println!("Original: {:?}", nums);
    println!("After filter/map/collect: {:?}\n", result);
}

fn enumerate_and_zip() {
    println!("5) enumerate() and zip()");

    let names = vec!["Alice", "Bob", "Charlie"];

    for (index, name) in names.iter().enumerate() {
        println!("Index {index}: {name}");
    }

    let ids = vec![101, 102, 103];

    for (name, id) in names.iter().zip(ids.iter()) {
        println!("{name} has id {id}");
    }

    println!();
}

fn custom_iterator_notes() {
    println!("6) Important ideas to remember");
    println!("- An iterator produces items one at a time.");
    println!("- next() returns Option<Item>.");
    println!("- Some(value) means there is another item.");
    println!("- None means the iterator is finished.");
    println!("- iter() borrows items.");
    println!("- iter_mut() mutably borrows items.");
    println!("- into_iter() moves ownership.");
    println!("- map/filter are lazy until consumed by sum/collect/for/etc.\n");

    println!("Mini mental model:");
    println!("collection -> iterator -> adapters (map/filter) -> consumer (collect/sum/for)");
}

/*

EXTRA EXPLANATION
=================

What is an iterator?
An iterator is anything that lets you process items one at a time.
Instead of manually indexing through a collection, Rust lets you use
iterator methods to express your logic more clearly.

Example:
    let v = vec![1, 2, 3];
    let it = v.iter();

The variable `it` is an iterator over references to the values in `v`.

Why are iterators useful?
- Cleaner code
- Safer than manual indexing in many cases
- Easy chaining with methods like map, filter, take, skip, zip
- Often optimized very well by Rust

Three very common ways to iterate
---------------------------------
1. iter()
   Borrows each item immutably.
   Item type usually looks like &T.

2. iter_mut()
   Borrows each item mutably.
   Item type usually looks like &mut T.

3. into_iter()
   Takes ownership of the collection.
   Item type is usually T.

Lazy behavior
-------------
Iterator adapters like map() and filter() do not run immediately.
They only describe a pipeline.
The work happens when you consume the iterator using things like:
- collect()
- sum()
- count()
- for loop
- next()

Example:
    let v = vec![1, 2, 3];
    let mapped = v.iter().map(|x| x * 2); // lazy
    let out: Vec<_> = mapped.collect();   // now it runs

A note on references
--------------------
When you use iter(), closures often receive references.
That is why you may see things like:
    .map(|x| x * 2)
where x is actually &i32 and Rust dereferences it for you.
Sometimes you may want:
    .copied()
or
    .cloned()
so you can work with owned values instead.

Good iterator methods to learn next
-----------------------------------
- map
- filter
- collect
- sum
- count
- find
- any
- all
- fold
- enumerate
- zip
- take
- skip

*/

