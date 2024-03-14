// Understanding Ownership

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
    
}