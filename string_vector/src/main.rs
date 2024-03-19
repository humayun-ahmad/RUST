use std::any::type_name_of_val;


fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

fn retrieving_value_from_vector(){
    let vec = vec![1,2,3,4,5];

    // Retrieve a value at a specific index
    let third_value = vec[2];
    println!("The third value in the vector is : {}", third_value);

    // Retrieve the last value
    let last_value = vec.last().unwrap();
    println!("The last value in the vector is : {}", last_value);

    //Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}\n", first_value),
        None => println!("The vector is empty!\n"),
    }
}

#[warn(dead_code)]
fn variable_type(){
    let sentence = "the quick brown fox jumps over the lazy dog";
    println!("{}", type_name_of_val(&sentence));
}

fn string_slice(sentence: &str){
    println!("{}", &sentence[0..=4]);

    let res = format!("Title: Quick story\n{}", sentence);

    for c in sentence.chars(){
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
            _ => continue,
        }
    }

    // For spliting and collect into a vector
    // Step-1:
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("{:#?}", words);

    // Step-2:
    let words1 = sentence.split(' ').collect::<Vec<&str>>();
    println!("Words1 \n {:#?}", words1);


    // Reversing the sentence
    let reversed = sentence.chars().rev().collect::<String>();
    println!("Reversed: {:#?} \n", reversed);
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
    // string_slice(sentence);

    let vec = vec![1, 2, 3, 4, 5];
    get_item(3);

    println!("Execution Compeleted!");
}




