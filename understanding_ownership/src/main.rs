fn main() {
    let s1 = String::from("Hello");

    // let (s2, len): (String, usize) = calculate_length(s1);

    // println!("The length of '{}' is {}.", s2, len);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);


    let mut s2 = s1.clone();
    {
        change(&mut s2);
    }
    let r2 = change(&mut s2);
    println!("The string is: {}", s2);


}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}


// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s,length)
// }
