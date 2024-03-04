
// Fix the error below with least amount of modification to the code
fn main() {
    let x: i32; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}


// Fill the blanks in the code to make it compile
fn demo1() {
    let __ __ = 1;
    __ += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}