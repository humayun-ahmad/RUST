// #[derive(Debug)]
// Define the Person struct before using it in any functions
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

// Define the function that creates an instance of Person
fn creating_struct_instance() {
    let alfredo = Person {
        first_name: "Alfredo".to_string(),
        last_name: "Sanchez".to_string(),
        age: 24,
    };

    // This will print the debug format of the alfredo instance
    println!("{:#?}", alfredo);
}

fn main() {
    println!("We are in a structs_types_enum project!");

    // Now calling the function after all relevant declarations
    creating_struct_instance();
}
