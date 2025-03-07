use std::collections::HashMap;
use std::io;
// Desired Functionality:
// 1. User can add employee names to a department.
// 2. User should be able to retrieve a list of all people in a department.
// 3. User should be able to retrieve all users by department sorted alphametically.

struct Department {
    name: String,
    employees: Vec<String>,
}



fn initialize_departments() -> HashMap<usize, Department> {
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

    for (id, name) in departments.iter().enumerate() {

        // Initialize a hashmap with department ids and their correpsonding deparments
        let department = Department {
            name: name.to_string(),
            employees: Vec::<String>::new()
        };

        // Store the department in a hashmap
        employees.insert(id + 1, department);
        
        // Display department and ID
        println!("{}: {name}", id + 1);
    }

    return employees

}

fn display_prompts() {
    let mut name: String = String::new();
    
    println!("Please type in a Name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // Trim removes \n but turns the String to &str so we must call to_string()
    let name: String = name.trim().to_string();

    println!("Please select a department to add {name} to:");
    let mut departments = initialize_departments();

}

fn main() {
    display_prompts();
}
