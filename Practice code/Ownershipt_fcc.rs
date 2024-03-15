
fn main() {
    // let s = "hello";
    let mut s = String::from("Hello");
    
    // push_str() appends a literal to a String
    s.push_str(", World!");
    
    println!("{}", s);
    
    
    let s1 = String::from("hello");
    
    let s2 = s1.clone();
    
    println!("{}",s2);
    
    
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    
    
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);
    
    
    
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.