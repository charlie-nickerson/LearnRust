// Desired Functionality:
// 1. User can add employee names to a department.
// 2. User should be able to retrieve a list of all people in a department.
// 3. User should be able to retrieve all users by department sorted alphametically.

struct Department {
    id: u32,
    name: String,
    employees: Vec<String>,
}



fn initialize_departments(){
    use std::collections::HashMap;
    let mut employees = HashMap::new();

    let departments = vec!["Human Resources",
    "Finance and Accounting",
    "Information Technology", 
    "Marketing and Communications",
    "Research and Development",
    "Operations",
    "Customer Service",
    "Sales",
    "Legal",
    "Supply Chain Management"];

    for department in departments.iter() {
        // Initialize a hashmap with department names as keys
        // and an empty vector that stores the employee names
        employees.insert(department, Vec::<String>::new());
    }    
}

fn display_prompts() {
    use std::io;
    let mut name: String = String::new();
    
    println!("Please type in a Name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // Trim removes \n but turns the String to &str so we must call to_string()
    let name: String = name.trim().to_string();

    println!("Please select a department to add {name} to:");


    

}

fn main() {
    display_prompts()
}
