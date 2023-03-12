fn main() {
    foo(5, '#');
    bar();

    let x: u32 = five();
    println!("The value of x: {x}");

    let y: u32 = plus_one(x);
    println!("The value of x + 1: {y}");
}

fn foo(x: u32, c: char) {
    println!("foo({x}, {c})");
}

fn bar() {
    // statements are instructions that perform some action and do not return a value.
    // expressions evaluate to a result value.
    let y = {
        let x = 3;
        // this is a statement, and does not need a semicolon
        x + 1
    };

    println!("The value of y: {y}");
}

// this function returns a u32.
// functions return the last statement/expression.
// the word return can be used to return early.
fn five() -> u32 {
    5
} 

// receives a parameters and returns x + 1.
fn plus_one(x: u32) -> u32 {
    x + 1
}