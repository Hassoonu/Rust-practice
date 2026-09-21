fn main(){
    // Rust code uses snake case as the conventional style for function and variable names, 
    // in which all letters are lowercase and underscores separate words.

    // Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller

    // blocks of code evaluate to the last expression in them
    func_with_param(8);

    func_with_more_params(12, "PM");

    statements_and_expressions();

    let y = return_value_functions();

    println!("Returned {y}");

    let y = modifying_function(y);

    println!("Modified to {y}");
} 

fn func_with_param(x:i32){
    // function parameters MUST have their type declared in the signature
    // reasoning: declaring it in the signature means you almost never have to 
    //      declare them elsewhere in the code for the compiler.
    //      Also more helpful error messages from the compiler.
    println!("Value of input is {x}");
}

fn func_with_more_params(x:i32, label: &str){
    // comma separated
    println!("Time is: {x}{label}");
}

fn statements_and_expressions(){
    // statements are instructions that perform some action and DO NOT return a value
    // ex: assignment, fuycntion definitions,

    // expressions evaluate to a resultant value
    // ex: 5 + 6

    // since statements dont return values, you cant do something like:
    // let x = let y = 6, or let x,y = 6

    // let (x, y) = (10, 20); // < reminder for tuples unit

    // expressions can be part of statements, ex: in let y = 6, 6 is an expression.

    // calling a function or a macro is an expression.

    // a new scope block created with curly brackets is an expression:

    let y = {
        let x = 6;
        x + 1
    };

    println!("y is {y}");
    // notice, we didnt adda semi-colon to x+1, and we added one at the end of the curly brackets.

    // expressions do not include the semicolons at the end of the line.
        // if you add a semicolon to the end of an expression, you turn it to a statement, 
        // and it wont return a value.
}

fn return_value_functions () -> i8 {
    12
}

fn modifying_function(x: i8) -> i8 {
    x + 1
    // x + 1; returns an error because the semicolon turns x + 1 to a statement, so the function
    // instead returns the unit type, (). Must remove the semicolon to turn x + 1 to an expression
}