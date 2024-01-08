use std::io;
use std::io::Write;

// program that prompts for age, and checks if it is under or over the legal age.
// Inputs: age
// Process: checks if legal age
// Outputs: "You are old enough to legally drive." or "You are not old enough to legally drive."

fn can_drive_legally(age: u32, legal_age: u32) -> bool {
    age >= legal_age
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_drive_legally() {
        assert_eq!(can_drive_legally(1, 16), false);
        assert_eq!(can_drive_legally(15, 16), false);
        assert_eq!(can_drive_legally(16, 16), true);
        assert_eq!(can_drive_legally(30, 16), true);
        assert_eq!(can_drive_legally(0, 16), false);
        assert_eq!(can_drive_legally(3252340, 16), true);
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    // Set legal age to 16
    const LEGAL_AGE: u32 = 16;
    // prompt for input_age "What is your age? "
    let input_age: u32 = get_input("What is your age? ");
    // Check if age is legal
        // if legal, print "You are old enough to legally drive."
        // if not, print  "You are not old enough to legally drive."
    if can_drive_legally(input_age, LEGAL_AGE) {
        println!("You are old enough to legally drive.");
    } else {
        println!("You are not old enough to legally drive.");
    }
}
