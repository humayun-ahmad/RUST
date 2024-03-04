
// Make it work
use std::mem::size_of_val;
fn variable_size() {
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1),4); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); 

    println!("Success!");
} 


// Make it work
fn char_argument_pass() {
    let c1: char = '中';
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}


// Make it work, don't modify `implicitly_ret_unit` !
fn unit_type() {
    let _v: () = ();

    let v: (u32, u32) = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}


// Modify `4` in assert to make it work
use std::mem::size_of_val;
fn unit_size() {
    let unit: () = ();
    // println!("{}", size_of_val(&unit));
    
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}

fn statement_expression() {
    let x: u32 = 5u32;

    let y: u32 = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z: u32 = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

// Make it work with two ways
fn statement_expression_2() {
    let v: i32 = {
        let mut x: i32 = 1;
        x += 2;
        x
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
 }

// Make it work with two ways
fn demo_exercise_1() {
    let v: i32 = {
        let mut x: i32 = 1;
        x += 2;
        x
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
 }


fn demo_exercise_2() {
    let mut v = {
             let x = 3;
             x
        
    };
 
    assert!(v == 3);
 
    println!("Success!");
 }


fn demo_exercise_3() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

// Solve it in two ways
// DON'T let `println!` work
fn never_return_function_1() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    
    panic!()
}


fn diverging_function_1() {
    get_option()

    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    // panic!()
    // unimplemented!();
    todo!();
}


fn diverging_function_2() {
    // FILL in the blank
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}



fn main(){
    variable_size();
    char_argument_pass();
    unit_type();
    println!("Success from main!");

}