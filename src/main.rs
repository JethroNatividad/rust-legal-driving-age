// program that prompts for age, and checks if it is under or over the legal age.
// Inputs: age
// Process: checks if legal age
// Outputs: “You are old enough to legally drive.” or “You are not old enough to legally drive.”

fn can_drive_legally(age: i32) -> bool {
    age >= 16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_drive_legally() {
        assert_eq!(can_drive_legally(1), false);
        assert_eq!(can_drive_legally(15), false);
        assert_eq!(can_drive_legally(16), true);
        assert_eq!(can_drive_legally(30), true);
        assert_eq!(can_drive_legally(0), false);
        assert_eq!(can_drive_legally(3252340), true);
    }
}


fn main() {
    println!("Hello, world!");
}
