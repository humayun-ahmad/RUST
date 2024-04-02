fn main() {
    let s1 = String::from("Hello world");

    // let (s2, len): (String, usize) = calculate_length(s1);

    // println!("The length of '{}' is {}.", s2, len);

    // let len = calculate_length(&s1);

    // println!("The length of '{}' is {}.", s1, len);


    let mut s2 = s1.clone();
    // {
    //     change(&mut s2);
    // }

    // let r2 = change(&mut s2);

    // println!("The string is: {}", s2);

    // another_function();

    // let s3 = first_word(&s2);

    // println!("{s3}");

    // second_word();

    let s4 = first_word_update(&s1);

    println!("Result is : {s4}\n");

    println!("immutable_borrow is calling now!");

    immutable_borrow();

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


fn another_function(){
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    
    // println!("{}, {}, and {}", r1, r2, r3);
    println!("{}, {}", r1, r2);
    let r3 = &mut s; // no problem
    println!("{r3}");
    
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn second_word() {
    let s = String::from("hello");

    let len = s.len();

    let slice1 = &s[0..2];
    let slice1 = &s[..2];


    let slice2 = &s[3..len];
    let slice2 = &s[3..];

    let slice3 = &s[0..len];
    let slice3 = &s[..];

    println!("Slice 1 : {slice1}, Slice 2 : {slice2}, Slice 3 : {slice3}");
}

fn first_word_update(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn immutable_borrow() {
    let mut s = String::from("hello world");

    let word = first_word_update(&s);

    // s.clear(); // error!

    println!("the first word is: {}", word);

    println!("immutable_borrow has finished!");
}