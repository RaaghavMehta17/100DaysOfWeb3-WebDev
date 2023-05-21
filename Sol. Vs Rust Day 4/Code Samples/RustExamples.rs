//Access Control in Rust Smart Contract

#[ink::contract(version = "0.14.0")]
pub struct MyContract {
    owner: AccountId,
}

impl MyContract {
    #[ink(constructor)]
    pub fn new() -> Self {
        let caller = Self::env().caller();
        Self { owner: caller }
    }

    #[ink(message)]
    #[ink(payable)]
    #[ink(access_control(owner))]
    pub fn change_owner(&mut self, new_owner: AccountId) {
        self.owner = new_owner;
    }
}

//Validate contract addresses

#[ink::contract(version = "0.14.0")]
pub struct MyContract {}

impl MyContract {
    #[ink(message, payable)]
    pub fn transfer_funds(&mut self, recipient: AccountId) {
        assert!(recipient != AccountId::default(), "Invalid recipient address.");

        // Use env().transfer() to send funds
        self.env().transfer(recipient, self.env().balance());
    }
}

//Avoid Reentrancy attacks
use ink_lang::contract;

#[ink::contract]
mod my_contract {
    use ink_lang::Mutex; // Importing the Mutex type from ink_lang crate

    #[ink(storage)]
    struct MyContract {
        locked: Mutex<bool>, // Declare a Mutex variable to track reentrancy status
    }

    impl MyContract {
        #[ink(constructor)]
        fn new() -> Self {
            Self {
                locked: Mutex::new(false), // Initialize the locked variable as false
            }
        }

        #[ink(message)]
        #[ink(payable)]
        fn withdraw(&self) {
            // Acquire a lock on the locked variable to ensure exclusive access
            let mut locked = self.locked.lock();

            if *locked {
                // Reentrancy attack detected
                // Handle the attack, e.g., revert the transaction or perform necessary actions
            } else {
                *locked = true; // Set the locked variable to true to prevent subsequent reentrant calls

                // Perform withdrawal logic here
                // ...

                *locked = false; // Reset the locked variable after the function execution is complete
            }
        }
    }
}
