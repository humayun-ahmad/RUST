use std::any::type_name_of_val;

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    println!("{}", type_name_of_val(&sentence)); 
}