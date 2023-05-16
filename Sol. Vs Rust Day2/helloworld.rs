//Import necessary modules
use ink_lang::contract;

// Define the smart contract
#[contract]
mod hello_world {
    // Define the methods for the contract
    #[ink(message)]
    fn hello(&self) -> String {
        String::from("Hello, World!")
    }
}
