#[allow(unused_variables)]


// Fix the error below with least amount of modification to the code


// Fill the blanks in the code to make it compile
fn new_function() {
    let mut x = 1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}


fn unused_variable(){
    let _x = 1;
}

// Destructuring assignments


fn destructuring_assignment() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success from destructuring assignment!");
} 

fn numbers(){
    let x: i32 = 5;
    let mut y:i32 = 6;

    y = x; 

    let _z:i32 = 10; 
    println!("Success from numbers!");
}

// Fill the blank
fn type_casting() {
    let v: u16 = 38_u8 as u16;

    println!("Success! and Value of v is: {}", v);
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}


// Fill the blanks to make it work
fn max_value_() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}

fn add_casting() {
    let v1:u16 = 251_u16 + 8_u16;
    let v2:i16 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
 }


// Modify `assert!` to make it work
fn addition() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}

fn floating_point() {
    let x: f32 = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z: f64 = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f32".to_string());
    println!("Success!");
}


fn assert_fix() {
    // first way to do that
    assert!(0.1_f32+0.2_f32==0.3_f32);

    // second way to do that
    assert!(0.1 as f32 + 0.2 as f32==0.3 as f32);
    
    println!("Success!");
}

// 🌟🌟 Two goals: 1. Modify assert! to make it work 2. Make println! output: 97 - 122
fn print_sum(){
    let mut sum:i32 = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c as i32);
    }

}

fn main() {
    destructuring_assignment();
    unused_variable();
    new_function();
    type_casting();
    let x:u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));
    add_casting();
    addition();
    floating_point();
    assert_fix(); 
    println!("Success from main!");
}

