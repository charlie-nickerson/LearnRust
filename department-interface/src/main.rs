// Desired Functionality:
// 1. User can add employee names to a department.
// 2. User should be able to retrieve a list of all people in a department.
// 3. User should be able to retrieve all users by department sorted alphabetically.

use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Department {
    name: String,
    employees: Vec<String>,
}



fn initialize_departments() -> HashMap<usize, Department> {
    let mut employees = HashMap::<usize, Department>::new();

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
    }

    return employees

}

fn add_employees_to_department(departments: &mut HashMap<usize, Department>) {
    let mut name: String = String::new();
    
    println!("Please type in a Name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // Trim removes \n but turns the String to &str so we must call to_string()
    let name: String = name.trim().to_string();

    println!("Please select a department to add {name} to:");

    // Create a vector to list out the departments by order of their key values
    let mut entries: Vec<(&usize, &Department)>  = departments.iter().collect();
    entries.sort_by_key(|entry|entry.0);
    
    // list all the departments
    for (id, department) in entries {
        println!("{id}: {:?}", department.name);
    }


    let mut choice =  String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: usize = choice.trim().parse().expect("Please type a number!");

    // Add the name to the specified department and sort the employees in alphabetical order
    departments.get_mut(&choice).unwrap().employees.push(name);
    departments.get_mut(&choice).unwrap().employees.sort();
}

fn list_employees_in_department(departments: &mut HashMap<usize, Department>) {
    println!("What department do you want to list?");

    // Create a vector to list out the departments by order of their key values
    let entries: &mut Vec<(&usize, &Department)>  = &mut departments.iter().collect();
    entries.sort_by_key(|entry|entry.0);
    
    // list all the departments
    for (id, department) in entries {
        println!("{id}: {:?}", department.name);
    }
    
    let mut choice =  String::new();
    
    io::stdin()
    .read_line(&mut choice)
    .expect("Failed to read line");
    
    let choice: usize = choice.trim().parse().expect("Please type a number!");
    
    println!("{:?}", departments.get_mut(&choice).unwrap().employees);
}

fn list_all_employees(departments: &mut HashMap<usize, Department>) {
    // Create a vector to list out the departments by order of their names values
    let entries: &mut Vec<(&usize, &Department)>  = &mut departments.iter().collect();
    entries.sort_by_key(|entry|entry.1.name.clone());

    // list all the departments
    for (_, department) in entries {
        println!("Department Name: {:?}\nEmployees: {:?}\n", department.name, department.employees);
    }
}

fn main() {
    let mut departments = initialize_departments();
    loop {
        let mut choice =  String::new();
        
        println!("Select an option:
        1. Add an employee
        2. List employees from department
        3. List all employees");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: usize = choice.trim().parse().expect("Please type a number!");
    
        if choice == 1 {
            add_employees_to_department(&mut departments);
        } else if choice == 2 {
            list_employees_in_department(&mut departments);
        } else if choice == 3 {
            list_all_employees(&mut departments);
        } else {
            println!("Invalid option");
        }
    }
}
