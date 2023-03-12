fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // cant use mut and change the variable because I would
    // be changing the type to.
    let spaces = "   ";
    let _spaces = spaces.len();

    // this code dont compile, because we cant reasign the
    // type of the variable that is initially a string to an int.
    // let mut spaces = "   ";
    // spaces = spaces.len();
    
}
