
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Instantiating of structs");

    let mut user1 = User{
        active : true,
        username: String::from("demoUser"),
        email: String::from("demo@gmail.com"),
        sign_in_count: 1,
    };

    user1.active = false;

    // println!("{:?}", user1)
}
