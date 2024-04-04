
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

    let user3 = build_user(String::from("email@gmail.com"), String::from("email"));

    println!("{}", user3.email);

    let user2 = User {
        active: user3.active,
        username: user3.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("{}", user2.username);

    let user4 = User {
        email: String::from("another@example.com"),
        ..user1
    };


    println!("Username: {}", user4.username);

}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}