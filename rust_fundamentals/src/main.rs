fn main(){
    // shadowing_variable();
    // optional_value();
    // loop_fn();
    // take_input();
    match_control_flow();
}


fn shadowing_variable(){

    // Way 1
    let mut height = 190;
    height = height - 20;

    let result = if height > 180 {
        "Tall"
    }else if height > 170 {
        "Average"
    } else{
        "Short"
    };

    println!("Result: {}", result);
    
    // Way - 2
    let health = if height < 180 {"good"} else {"unknown"};
    println!("Result: {}", health);
    
    // Way - 3
    let health = if height < 180 {true} else {false};
    println!("Result: {}", health);

}

fn optional_value() {
    let maybe_number = Some(42); // or None in some cases
    if let Some(num) = maybe_number {
        println!("The number is {}", num);
    } else {
        println!("No number provided.");
    }
}

fn loop_fn(){
    let mut x = 1;
     loop{
        println!("x is {}", x);
        x += 1;
        if x > 5 {
            break;
        }
     }
}

use std::io;

fn take_input(){
    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a word (type 'stop' to exit): ");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        println!("You entered : {}", input);
    }

    println!("Goodbye!");
}

fn match_control_flow(){
    let mut name = String::new();
    println!("Enter a name: ");
    io::stdin().read_line(&mut name).expect("Faild to read input");

    match name.trim() {
        "Goodbye" => println!("Sorry to see you go."),
        "Hello" => println!("Hi, nice to meet you!"),
        _ => println!("I can't find a greeting, good bye."),
    }
}