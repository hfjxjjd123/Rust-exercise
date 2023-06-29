// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>


fn main() {
    let lockers: Vec<Locker> = vec![
        Locker{
            name: "Hakrim Lee".to_owned(), 
            assignment: Some(50)
        },
        Locker{
            name: "Jisoo Lee".to_owned(), 
            assignment: None
        },
        Locker{
            name:"GangIn Lee".to_owned(), 
            assignment: Some(18)},
    ];

    for locker in lockers {
        match locker.assignment{
            Some(num) => println!("{:?} has {:?}th locker", &locker.name, num),
            None => println!("{:?} has no locker", &locker.name),
        }
    }
}

struct Locker{
    name: String,
    assignment: Option<i32>
}