
fn print_str(s: &str){
    println!("{}", s);
}

fn print_string(s: String){
    println!("{}", s);
}

fn main() {
    let s = "Hello world";
    print_str(s);

    let salutation = String::from("Hello");
    print_string(salutation);

    println!("Execution Compeleted!");
}
