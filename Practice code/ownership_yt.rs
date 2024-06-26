// Ownership Rules:
// Each value in the rust has a variable that is its owner.
// A value can have only owner at a time
// When the owner goes out of scope, the value is dropped.

#[allow(unused_variables)]

fn main(){
    // Stack
    // - Fast memory creation and retrieval.... speed, speed and Speed
    // - Memory is automatically recaptured by the program after variables go out of scope
    // - Is the default in RUST
    // - Fixed size variable... Colections cannot be stack based(and Strings are collection of u8's)

    let stack_i8 : i8 = 10;
    let stack_f32 : f32 = 20.;
    let stack_bool : bool = true;
    let stack_char : char = 'a';


    // Heap 
    // - Flexibility
    // - Memory that can grow in size (vector, HashMap, String, etc)
    // - Runtime performance cost(speed)
    // - Memory that cn live beyond the scope that created it
    // - Memory is automatically recaptured when the last OWNER goes out of the scope

    let heap_vector: Vec<i8> = Vec::new();
    let heap_string: String = String::from("Raib");
    let heap_i8: Box<i8> = Box::new(30);


    let stack_i8_2 = stack_i8;
    println!("{}", stack_i8);
    println!("{}", stack_i8_2);
    


    let heap_i8_2 = heap_i8.clone();
    println!("{}", heap_i8);
    println!("{}", heap_i8_2);


    let stack_f64: f64 = 1.;
    let heap_f64: Box<f64> = Box::new(2.);

    
    stack_procedure(&stack_f64);
    println!("In main stack {}", stack_f64);
    
    heap_procedure(&heap_f64);
    println!("In main heap {}", heap_f64);

    let some_string: String = String::from("Rajib"); // String are always on the heap
    let some_str: &str = "Partner"; // &str is a pointer... to either stack or heap
    
    some_procedure(&some_string, some_str);

    println!("{} {}", some_string, some_str);

}


fn stack_procedure(param: &f64){
    println!("In stack_procedure with param {}", param);
}

fn heap_procedure(param: &Box<f64>){
    println!("In heap_procedure with param {}", param);
}

fn some_procedure(param_a: &String, param_b: &str){
    println!("In heap_procedure with param {} {}", param_a, param_b);
}
 



