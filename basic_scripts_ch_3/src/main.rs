const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    mutability_err_ex();
    println!("-------------");
    mutability_good_ex();
    println!("-------------");
    shadowing_variables();
    println!("-------------");
}

fn mutability_err_ex() {
    // Variables and Mutability:
    println!("Mutability Error:");
    let x = 5;
    println!("The value of x is {x}");

    // below generates a compiler error :(
    // x = 6;
    // println!("The value of x is {x}");
}

fn mutability_good_ex() {
    // Variables and Mutability:
    println!("Mutability Correct:");
    let mut y = 10;

    println!("y is {y}");

    y = 21;

    println!("y is NOW {y} ;)");
}

fn shadowing_variables() {
    println!("Shadowing:");
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("x is: {x}");
    }

    println!("after scope, x is now {x}");

    // a use case also lets us:
    // perform some transformations for a non-mutable variable and then keep it immutable
    // can re-use variable names for variables of different types. Ex:
    // let spaces = "    "; string type
    // let spaces = spaces.len(); int type

    // how does this affect memory?
}
