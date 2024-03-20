
fn main() {
    // example_creating_using_struct();
    example_struct_and_functions();
}



struct Employee{
    name:String,
    salary:u64,
    fulltime:bool,
}


fn example_creating_using_struct(){
    println!("Struct is here");

    let employe1 = Employee{
        name:String::from("Rajib"),
        fulltime: false,
        salary:2000
    };

    let mut employe2 = Employee{
        name:String::from("Maimuna"),
        fulltime: true,
        salary:500
    };

    employe2.salary *= 2;

    println!("{} earns ${}, fulltime status: {}", employe1.name, employe1.salary, employe1.fulltime);
    println!("{} earns ${}, fulltime status: {}", employe2.name, employe2.salary, employe2.fulltime);
}

fn example_struct_and_functions(){
    let employe3 = build_employee(String::from("Nawfaa"),3000, true);
    print_employee(&employe3);

    let employe4 = build_employee(String::from("Sung"),4000, true);
    print_employee(&employe4);
}

fn build_employee (name: String, salary: u64, fulltime: bool)-> Employee{
    Employee{
        name: name,
        salary: salary,
        fulltime: fulltime
    }
}


fn build_employee_v2 (name: String, salary: u64, fulltime: bool)-> Employee{
    Employee{
        name,
        salary,
        fulltime
    }
}


fn print_employee(emp: &Employee){
    println!("{} earns {}, fulltime status: {}", emp.name, emp.salary, emp.fulltime);
}




