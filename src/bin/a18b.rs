// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

fn main() {
let employees: Vec<Employee> = vec![
    Employee::new(EmployeeType::Manager, true, "Hakrim Lee"),
    Employee::new(EmployeeType::AssemblyTechnician, true, "Songess"),
    Employee::new(EmployeeType::KitchenStaff, false, "juki"),
    Employee::new(EmployeeType::Manager, false, "Cho gu seong"),
    Employee::new(EmployeeType::MaintenanceCrew, true, "Jaeho Yang"),
    Employee::new(EmployeeType::LineSupervisor, true, "Hakrim Min"),
];

for employee in employees{
    let validate = validation(employee);
    match validate{
        Err(e) => println!("{:?}", e),
        _ => {}
    }
}

}

enum EmployeeType{
    MaintenanceCrew,
    MarketingDepartment,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician
}

struct Employee{
    types: EmployeeType,
    current: bool,
    name: String,
}
impl Employee{
    fn new(types: EmployeeType, current: bool, name: &str) -> Self{
        Self{
            types: types,
            current: current,
            name: name.to_owned()
        }
    }
}

fn valid_enter(employee: Employee) -> Result<Employee,String> {
    if !employee.current {
        Err("NOT ALLOWED - expired".to_owned())
    } else{
        match employee.types {
            EmployeeType::MaintenanceCrew | EmployeeType::MarketingDepartment | EmployeeType::Manager 
                => Ok(employee),
            _ => Err("NOT ALLOWED - authority".to_owned()) 
        }
    }
}

fn validation(employee: Employee) -> Result<(),String> {
    let enter_employee: Employee = valid_enter(employee)?;
    println!("{:?} can pass", enter_employee.name);
    Ok(())
}