use std::collections::HashMap;
use std::io;

fn main() {
    let mut my_map = HashMap::new();

    loop {
        // Get user input for key
        let mut key = String::new();
        println!("Enter a key (or type 'exit' to finish):");
        io::stdin().read_line(&mut key).expect("Failed to read line");
        let key = key.trim();

        if key == "exit" {
            break;
        }

        // Get user input for value
        let mut value = String::new();
        println!("Enter a numeric value:");
        io::stdin().read_line(&mut value).expect("Failed to read line");
        // Parse the numeric value as an integer
        let value: i32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input for value. Please enter a numeric value.");
                continue;
            }
        };

        // Insert the key-value pair into the HashMap
        my_map.insert(key.to_string(), value);

        println!("Key-value pair inserted into the HashMap.");
    }

    // Sort the HashMap entries by value (descending) and then by key (ascending)
    let mut sorted_entries: Vec<_> = my_map.into_iter().collect();
    sorted_entries.sort_by(|a: &(String, i32), b: &(String, i32)| {
        // Sort by value (descending)
        b.1.cmp(&a.1)
            // If values are equal, sort by key (ascending)
            .then_with(|| a.0.cmp(&b.0))
    });

    // Print the sorted entries
    println!("Sorted HashMap:");
    for (key, value) in sorted_entries {
        println!("Key: {}, Value: {}", key, value);
    }
}
