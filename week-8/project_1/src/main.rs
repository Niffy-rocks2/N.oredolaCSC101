use std::collections::HashMap;

fn main() {
    // Create a HashMap to store APS levels based on position and experience
    let mut aps_levels: HashMap<(String, u32), u32> = HashMap::new();

    // Populate the HashMap with data from the table
    aps_levels.insert(("Intern".to_string(), 0), 1);
    aps_levels.insert(("Intern".to_string(), 1), 2);
    aps_levels.insert(("Paralegal".to_string(), 0), 1);
    aps_levels.insert(("Paralegal".to_string(), 1), 2);
    // ... Add more entries from the table

    // Get input from the user
    let position = String::from("Associate Lawyer");
    let years_of_experience = 5;

    // Check the APS level
    let aps_level = aps_levels.get(&(position, years_of_experience));

    if let Some(level) = aps_level {
        println!("The staff level is APS {}", level);
    } else {
        println!("Staff level not found.");
    }
}