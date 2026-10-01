fn main() -> () {
    

    let mut s2;
    {
        let mut s1 = String::from("test deez");
        println!("s1 is {s1}");
        s2 = s1;
        // s1.push_str(", world!"); // leads to error
        // "In addition, there’s a design choice that’s implied by this: Rust will never automatically create 
        //      “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance."
    }
    println!("s2 is {s2}"); // string still exists after scrope
}


// note2:
/*
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!"); // prints "ahoy, world!"

    for the code above: 
    We initially declare a variable s and bind it to a String with the value "hello". 
    Then, we immediately create a new String with the value "ahoy" and assign it to s. 
    At this point, nothing is referring to the original value on the heap at all.
    The original string thus immediately goes out of scope. 
    Rust will run the drop function on it and its memory will be freed right away. 
    When we print the value at the end, it will be "ahoy, world!".
*/