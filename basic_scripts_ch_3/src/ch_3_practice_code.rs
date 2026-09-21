// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
use std::io; // get io from user

fn get_input() -> String{
    // read_line appends to string, this ensures there's nothing before.
    let mut input = String::new();
    let input_size = io::stdin()
        .read_line(&mut input)
        .expect("Unable to get option!");

    if input_size == 0 {
        // no more input to read, user wants to exit:
        println!("Input closed. Exiting...");
        std::process::exit(0);
    }

    input
}

fn main() {
    println!("Which program would you like to run?\n1 is converting between Fahrenheit and Celsius.\n2 is to generate the nth Fibonacci number.\n3 is to sing for you!\nOr 0 to exit.");

    loop {
        let option = get_input();

        match option.trim() {
            "1" => { convert_between_fahr_and_cels(); break; }
            "2" => { nth_fib_num(); break; }
            "3" => { sing(); break; }
            "0" | "" => break, // `|` combines patterns in one arm
            _ => println!("Not a valid option, please try again!"),
        }
    }
}

fn convert_between_fahr_and_cels(){
    println!("Would you like to convert input number to Fahrenheit (F) or Celsius (C)?");

    'outer_loop: loop {
        let f_to_c = get_input();

        let f_to_c: bool = match f_to_c.trim().to_ascii_uppercase().as_str() {
            "C" => true,
            "F" => false,
            ""  => return,
            _ => { println!("Invalid option, please enter F or C."); continue }
        };

        println!("Value to convert:");

        'inner_loop: loop {
            let input_val = get_input();

            let mut value_to_convert: f32 = match input_val.trim().parse() {
                Ok(choice) => choice,
                Err(_) => {
                    println!("Invalid value, please try again.");
                    continue
                }
            };

            if f_to_c {
                value_to_convert = (value_to_convert - 32.0) * 5.0 / 9.0;
            }
            else {
                value_to_convert = ((value_to_convert * 9.0) / 5.0) + 32.0;
            }

            println!("New value is: {value_to_convert}");

            break 'outer_loop;
        }
    }
}

fn nth_fib_num(){
    println!("What n do you want to calculate to? n > 0 and n < 15");

    'fib_loop: loop {

        let fib_val_in = get_input();

        let fib_val: u32 = match fib_val_in.trim().parse::<u32>() {
            Ok(ans) => {
                if !(1..=14).contains(&ans) {
                    // above check is efficient for memory space and has good compiler optimizations.
                    println!("Not a valid value! Try again:");
                    continue
                }
                else {
                    nth_fib_helper(ans) // can convert ans to u32 (if it was u8) with: ans.into()
                }
            },
            Err(_) => {
                println!("Not a valid value! Try again:");
                continue
            }
        };

        println!("Result: {fib_val}");

        return;
    }
}

fn nth_fib_helper(n: u32) -> u32 {
    if (n == 1) || (n == 0) { // why does rust not allow parenthesis to encapsulate full if cond?
        return n;
    }
    else {
        nth_fib_helper(n - 1) + nth_fib_helper(n - 2)
    }
}

fn sing(){
    // variations:
    const STUFF_PER_DAY: [&str; 12] = ["Partridge in a Pear Tree",
                                    "Turtle Doves",
                                    "French Hens",
                                    "Calling Birds",
                                    "Gold Rings",
                                    "Geese a-laying",
                                    "Swans a-swimming",
                                    "Maids a-milking",
                                    "Ladies Dancing",
                                    "Lords a-leaping",
                                    "Pipers Piping",
                                    "Drummers Drumming"];

    let mut total_things = String::new();
    for day in 1..=12usize {
        let line = format!("{} {}\n", sing_helper_int_to_word(day as u8), STUFF_PER_DAY[day - 1]);
        total_things.insert_str(0, &line);

        let suffix = match day {1 => "st", 2 => "nd", 3 => "rd", _ => "th"};

        // let prefix_num = sing_helper_int_to_word(day as u8);
        // let mut today_we_got = day_stuff[day - 1].clone();
        // today_we_got.insert_str(0, &prefix_num);
        // today_we_got.push('\n');
        // total_things.insert_str(0, &today_we_got);

        // let ith = if day == 1 {
        //     "st"
        // } else if day == 2 {
        //     "nd"
        // }
        // else if day == 3 {
        //     "rd"
        // }
        // else {
        //     "th"
        // };
        

        println!("On the {day}{suffix} day of Christmas my true love sent to me\n{total_things}");
    }
}

fn sing_helper_int_to_word(x: u8) -> &'static str{
    return match x {
        1 =>       "A ",
        2 =>     "Two ",
        3 =>   "Three ",
        4 =>    "Four ",
        5 =>    "Five ",
        6 =>     "Six ",
        7 =>   "Seven ",
        8 =>   "Eight ",
        9 =>    "Nine ",
        10 =>    "Ten ",
        11 => "Eleven ",
        12 => "Twelve ",
        _ => unreachable!("day must be 1..=12"),
    }
}