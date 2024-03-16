fn main() {
    // example_enum_with_data();
    // example_enum_with_function();
    example_vec();
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

fn example_enum_with_function(){
    let favnum: Option<i32>;

    favnum = Option:: Some(107);

    match favnum {
        Some(n) => println!("My favorite number is {}, good choice", n),
        None => println!("What is the fuchka?"),
    }
}

fn example_vec(){
    println!("Using a vector");

    let mut vec1: Vec<i32> = Vec::new();
    let mut _vec1b=Vec::<i32>::new();

    vec1.push(20);
    vec1.push(101);
    vec1.push(100);

    println!("vec1 is {:?}, length is {}, first element is {}", vec1, vec1.len(), vec1[0]);

    let mut vec2 = vec!["Dhaka", "Barisal", "khulna"];

    vec2.insert(0, "Rajshahi");

    for item in vec2 {
        println!("{}", item);
    }
}