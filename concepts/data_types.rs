
//Integer Types
/*


Length	Signed	Unsigned
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
Architecture-dependent	isize	usize

*/

//Floating-Point Types

let x = 2.0; // f64
let y: f32 = 3.0; // f32

// Tuple type

let tup : (i32, f64, u8) = (500, 6.4,1);
let five_hundred = tup.0;
let sixe_point_four = tup.1;

let a: [i32; 5] = [1,2,3,4,5];

let b = [3;5]; // -> [3,3,3,3,3]

// Slice Types

/*

Slices let you refernce a contingous sequence of elements in a collection.
A slice is a kind of refernce, so it does not have ownership
*/

//Option

enum Option<T> {
    None,
    Some(T),
}

let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y; //ERROR

//way 1
let sum = x + y.unwrap(); //will panic if y is None

//way 2
let sum = match y {
    Some(value) => x + value,
    None => x,
};

//way 3
let sum = x + y.unwrap_or(0);