// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

fn main() {
   
    loop {
        let change_state = operate_state();
        match change_state{
            Ok(_) => {}
            Err(_) => println!("read is failed"),
        }   
    }
    
}

enum State {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate
}
impl State {
    fn state_operating(word: &str) -> Option<State>{
        use State::*;

        match word{
            "off" => Some(Off),
            "sleep" => Some(Sleep),
            "reboot" => Some(Reboot),
            "shutdown" => Some(Shutdown),
            "hibernate" => Some(Hibernate),
            _ => None,
        }
    }
}

fn get_input() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    
    Ok(buffer.trim().to_lowercase().to_owned())
}

fn operate_state() -> Result<(), io::Error>{
    use State::*;

    let input = get_input()?;
    let state: Option<State> = State::state_operating(&input);

    match state{
        Some(state) => {
            match state{
                Off => println!("off the screen"),
                Sleep => println!("Sleep Mode"),
                Reboot => println!("rebooting..."),
                Shutdown => println!("shutdown after 10secs"),
                Hibernate => println!("hibernating"),
            }
        }
        None => println!("{:?} is not keyword", input),
    }

    Ok(())
}