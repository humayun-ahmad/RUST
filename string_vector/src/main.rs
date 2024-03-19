use std::any::type_name_of_val;

fn variable_type(){
    let sentence = "the quick brown fox jumps over the lazy dog";
    println!("{}", type_name_of_val(&sentence));
}

fn string_slice(sentence: &str){
    println!("{}", &sentence[0..=4]);
}

fn print_str(s: &str){

    // For changing string we can do various thing
    // Step 1 
    let new_string_1 = format!("{}! other stuff here", s);
    println!("{}", new_string_1);

    // Step 2
    let mut new_string_2 = s.to_string();
    new_string_2.push_str(" string 2");
    println!("{}", new_string_2);
}

fn print_string(s: String){
    println!("{}", s);
}

fn main() {
    let s = "Hello world";
    // print_str(s);

    let salutation = String::from("Hello");
    // print_string(salutation);

    // variable_type();
    let sentence = "the quick brown fox jumps over the lazy dog";
    string_slice(sentence);
    println!("Execution Compeleted!");
}




