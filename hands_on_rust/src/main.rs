fn main() {
    example_enum_with_data();
}


enum Address{
    Number(i32),
    Name(String),
    Unknown
}


fn example_enum_with_data(){
    println!("\nexample enums with data");
    let h: Address = Address::Number(4);

    match h {
        Address::Number(n) => println!("You live in house number {}", n),
        Address::Name(s) => println!("You live in a house named {}", s),
        Address::Unknown => println!("Your house location is unknown"),
    }

    println!("Btw the size of Address is {} bytes", std::mem::size_of::<Address>());
}
