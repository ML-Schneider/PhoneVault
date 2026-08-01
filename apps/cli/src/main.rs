use std::env;

use phonevault_core::vault::initializer::VaultInitializer;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("phonevault init <location>");
        return;
    }

    let command = &args[1];
    let location = &args[2];

    match command.as_str() {

        "init" => {
            match VaultInitializer::initialize(location) {
                Ok(_) => {
                    println!("PhoneVault created successfully.");
                }

                Err(error) => {
                    println!("Error creating PhoneVault: {:?}", error);
                }
            }
        }

        _ => {
            println!("Unknown command.");
        }
    }
}