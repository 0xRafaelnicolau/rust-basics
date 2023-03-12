fn main() {
    // SCALAR TYPES

    // float-point type
    let _a = 2.0; // default is f64
    let _b: f32 = 3.0; // this is f32

    // numeric operations
    // this results in -1 (-1.666 is rounded down to 0)
    let _truncated = -5 / 3; 
    
    // booleans
    let _t = true;
    // with type annotation  
    let _f: bool = false;


    // COMPOUND TYPES

    // TUPLE TYPE
    // this type is used to group values of different types
    // tuples have fixed-lenght once declared they cant change in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x: {x}, y: {y}, z: {z}.");

    let tup: (u32, u8, u8) = (256, 255, 50);
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("The value of a: {a}, b: {b}, c: {c}.");

    // ARRAY TYPE
    // this types requires that every element is of the same type.
    // stores the data on the stack and not on the heap.
    // has a fixed number of elements.
    let _arr1 = [1, 2, 3, 4, 5];
    let _arr2: [u8; 5] = [2, 4, 6, 8, 10]; 
    let _arr3 = [3; 5]; // [3, 3, 3, 3, 3] 

}
