// notes and questions:
/*
Rust is a statically typed language, so it needs to know all data types
    at compile type. Compiler can usually infer what type we want based
    on how we use the variable. In cases where many types are possible, 
    we must add a type annotation like this:
    let guess: u32 = "42".parse().expect("Not a number!");

    another way to write this is using the 'turbofish':
    let guess = "42".parse()::<u32>();



    Can write integer literals in any of the forms in the Number literals table.

    Number literals that can be multiple types sallow a type suffix, ex:
    69u8

    Number literals also can use _ as a visual separator for ease of reading.
    so 1_000_000 is the same as 1000000
*/

fn main(){
    scalar_types();

    integer_literals();

    floating_point_types();

    numeric_operations();

    booleans();

    character_type();

    compound_data_types();
}


fn scalar_types(){
    println!("Scalar Types:");
    println!("Build-in integer types in Rust:");
    println!("| Length                 | Signed | Unsigned |");
    println!("| 8-bit                  | i8     | u8       |");
    println!("| 16-bit                 | i16    | u16      |");
    println!("| 32-bit                 | i32    | u32      |");
    println!("| 64-bit                 | i64    | u64      |");
    println!("| 128-bit                | i128   | u128     |");
    println!("| Architecture-dependent | isize  | usize    |");

    println!("Signed numbers can represent numbers from -2^(n-1) to 2^(n-1) - 1 inclusive.");
    println!("Unsigned numbers can represent numbers from 0 to 2^n - 1 inclusive.");
}

fn integer_literals(){
    println!("Integer Literals:");
    println!("Forms for integer literals:");
    println!("| Number Literals | Example     |");
    println!("| Decimal         | 98_222_123  |");
    println!("| Hex             | 0xff        |");
    println!("| Octal           | 0o77        |");
    println!("| Binary          | 0b1111_0000 |");
    println!("| Byte (u8 only)  | b'A'        |");

    // let mut test: u8 = 255;

    // test = test + 10;

    // println!("Test is {test}");
}

/* Integer Overflow behavior:
When in debug mode, rust checks for it and panics at runtime. DOES NOT CHECK AT COMPILE TIME
by panicing, rust only aborts and shows an error message.

When in release mode, program does NOT panic, and instead performs two's complement wrapping.

So you go from max value to minimum value.

To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:
    Wrap in all modes with the wrapping_* methods, such as wrapping_add.
    Return the None value if there is overflow with the checked_* methods.
    Return the value and a Boolean indicating whether there was overflow with the overflowing_* methods.
    Saturate at the value’s minimum or maximum values with the saturating_* methods.

*/

fn floating_point_types(){
    println!("Floating point data types:");
    let x = 2.0; // f64
    let y: f32 = 6.9;

    println!("Build-in floating point types in Rust:");
    println!("| Length                 | Signed |");
    println!("| 32-bit                 | f32    |");
    println!("| 64-bit                 | f64    |");

    // rust has only 2 primitive floating point numbers, f32 and f64.
    // ALL floating-point types are signed. Default is f64 bcs its 
    // about the same speed as f32 and has more precision.

    //Floating-point numbers are represented according to the IEEE-754 standard.
}

fn numeric_operations(){
    // addition
    let sum = 5 + 12;

    // subtraction
    let diff = 64 - 32;

    // multiplication
    let product = 12 * 5;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // results in -1;

    // modulus
    let remainder = 43 % 5;
}

fn booleans(){
    let t = true;

    let f: bool = false;

    // one byte
}

fn character_type(){
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // char literals are declared with single quotes, as opposed to string literals, which use double quotation marks

    // char type is 4 bytes in size and represents a Unicode scalar value.
        //which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emojis; and zero-width spaces are all valid char values in Rust

    // Unicode scalar values range from U+0000 to U+D7FF, and U+E000 to U+10FFF
}

fn compound_data_types(){
    // group multiple VALUES to one TYPE
    // Rust has 2 primitive compound data types: Tuples and Arrays
    // TUPLES:

    // Tuples have a fixed length: Once declared, they cannot grow or shrink in size.
    // We create a tuple by writing a comma-separated list of values inside parentheses. 
    // Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // or 
    let tup = (501, 6.5, 2);

    let (x, y, z) = tup;

    println!("The values of x, y, z is: {x}, {y}, {z}");

    //This program first creates a tuple and binds it to the variable tup. 
        // It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z.

    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access. For example:

    let five_o_one = tup.0;
    let six_point_five = tup.1;
    let two = tup.2;

    println!("five_o_one: {five_o_one}");
    println!("six_point_five: {six_point_five}");
    println!("two: {two}");

    // The tuple without any values has a special name, unit. 
    // This value and its corresponding type are both written () and represent an empty value or an empty return type. 
    // Expressions implicitly return the unit value if they don’t return any other value.

    // ARRAYS:
    // Another way to have a collection of multiple values is with an array. 
        //Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

    let arr = [1, 2, 3, 4, 5];
    // Arrays are useful when you want your data allocated on the stack, 
    //  the same as the other types we have seen so far, rather than the heap 
    //  or when you want to ensure that you always have a fixed number of elements

    // A vector is a similar collection type provided by the standard library 
        // that is allowed to grow or shrink in size because its contents live on the heap. 
        // If you’re unsure whether to use an array or a vector, chances are you should use a vector.
    
    // However, arrays are more useful when you know the number of elements will not need to change. 
    // For example, if you were using the names of the month in a program, 
    // you would probably use an array rather than a vector because you know it will always contain 12 elements:
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // You write an array’s type using square brackets with the type of each element, 
    // a semicolon, and then the number of elements in the array, like so:

    let arr_2: [u8;5] = [1, 2, 4, 56, 7];

    // You can also initialize an array to contain the same value for each element 
    // by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, 
    // as shown here:

    let arr_init = [0; 5]; // same as [0, 0, 0, 0, 0]

    // An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. 

    let arr_acc = [1,2,3,4,5,56,7,8];

    let fir = a[0];
    let sec = a[1];
}

