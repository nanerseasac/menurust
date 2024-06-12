use std::io::{self, Write};


fn main() {
    // program_loop();
    program_loop();
}

fn program_loop() {
    let mut arr: Vec<String> = Vec::new(); // Armazena os itens como String

    loop {
        println!("###########################");
        println!("Press 1 to insert an item");
        println!("Press 2 to remove an item");
        println!("Press 3 to list");
        println!("Press 4 to leave");
        println!("###########################");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim(); // Remove any trailing newline character

        match input {
            "1" => {
                print!("Enter item to insert: ");
                io::stdout().flush().unwrap();
                let mut item = String::new();
                io::stdin()
                    .read_line(&mut item)
                    .expect("Failed to read line");
                let item = item.trim().to_string(); // Remove trailing newline and convert to String
                arr.push(item);
                println!("Item inserted.");
            }
            "2" => {
                if arr.is_empty() {
                    println!("Array is empty. Nothing to remove.");
                } else {
                    print!("Enter index to remove: ");
                    io::stdout().flush().unwrap();
                    let mut index_str = String::new();
                    io::stdin()
                        .read_line(&mut index_str)
                        .expect("Failed to read line");
                    match index_str.trim().parse::<usize>() {
                        Ok(index) => {
                            if index < arr.len() {
                                arr.remove(index);
                                println!("Item removed.");
                            } else {
                                println!("Index out of bounds.");
                            }
                        }
                        Err(_) => println!("Invalid index."),
                    }
                }
            }
            "3" => {
                println!("Current items: {:?}", arr);
            }
            "4" => {
                println!("Exiting program.");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}

