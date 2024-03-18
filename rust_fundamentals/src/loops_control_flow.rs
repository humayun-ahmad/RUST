fn main(){
    optional_value();
}


fn optional_value() {
    let maybe_number = Some(42); // or None in some cases
    if let Some(num) = maybe_number {
        println!("The number is {}", num);
    } else {
        println!("No number provided.");
    }
}