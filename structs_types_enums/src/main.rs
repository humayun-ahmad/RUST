#[derive(Debug)]
// Define the Person struct before using it in any functions
struct Person {
    first_name: String,
    last_name: String,
    age: Option<u8>,
}

// Define the function that creates an instance of Person
fn creating_struct_instance() {
    let alfredo = Person {
        first_name: "Alfredo".to_string(),
        last_name: "Sanchez".to_string(),
        age: Some(24),
        // age : None,
    };

    // This will print the debug format of the alfredo instance
    println!("{:#?}", alfredo);

    println!("The person's first name is : {}", alfredo.first_name);
}

struct User{
    username : String,
    email : String,
    uri : String,
    active: bool
}

impl User{
    fn new(username: String, email: String, uri: String) -> Self{
        Self{
            username,
            email,
            uri,
            active: true,
        }
    }

    fn deactivate(&mut self){
        self.active = false;
    }
}


fn associated_constructors(){
    let mut new_user = User::new(
        String::from("Rajib"),
        String::from("rajib@example.com"),
        String::from("http://rajib.com"),
    );

    println!("Hello {}", new_user.username);
    println!("Acount {} status is : {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Acount {} status is : {}", new_user.username, new_user.active);
    

    
}

fn main() {
    println!("We are in a structs_types_enum project!");

    // Now calling the function after all relevant declarations
    // creating_struct_instance();
    
    //  Associated functions and constructors
    associated_constructors();

}
