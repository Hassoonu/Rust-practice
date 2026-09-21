fn main() {
    if_cond(10);
    if_in_let();
    loops();
    loop_labels();
    while_block();
    for_block();
}

fn if_cond(x: i8){

    if x > 5 {
        println!("I think the number is bigger than 5");
    } else{
        println!("I DON'T think the number is bigger than 5");
    }


    // commented code below returns an error. If statements expect a BOOLEAN!!!!!
    // let number = 3;

    // if number {
    //     println!("number exists!");
    // }

    // multiple if conditions:
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_let(){
    let condition = true;
    let number = if condition { 6 } else { 7 };

    println!("Number is {number}");

    // the values that have the potential to be results from each arm of the if must be the same type

    //ex: the below code returns an error bcs 6 is an int and "six" is a str
    // let number = if condition { 6 } else { "six" };
    /*
    Rust needs to know definitively at compile time what type the number variable is. 
    Knowing the type of number lets the compiler verify the type is valid everywhere we use number. 
    Rust wouldn’t be able to do that if the type of number was only determined at runtime; 
    the compiler would be more complex and would make fewer guarantees 
    about the code if it had to keep track of multiple hypothetical types for any variable.
    */
}

fn loops(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // after break, it appears we return counter * 2, we add semi-colon why tho?
            break counter * 2;
        }
        println!("I will NEVER STOP!");
    };

    println!("Result is {result}");
}

fn loop_labels(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_block(){
    let mut counter = 3;

    while counter != 0 {
        println!("counter is: {counter}");

        counter -= 1;
    }
}

fn for_block() {
    let arr = [10, 20, 30, 40, 50, 60, 70];

    for elem in arr {
        println!("elem in arr is {elem}");
    }

    // since for is so concise and safe, its one of teh most
    // often used iterating blocks in rust. For the count down we did
    // in the while_block, we can implement it like the below:

    for num in (1..4).rev() {
        println!("num!");
    }
    println!("FINISHED!");
}