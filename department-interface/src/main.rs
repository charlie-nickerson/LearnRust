// Desired Functionality:
// 1. User can add employee names to a department.
// 2. User should be able to retrieve a list of all people in a department.
// 3. User should be able to retrieve all users by department sorted alphametically.

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

fn main() {
    println!("Hello, world!");
}
