//(1) Avoiding Redundant State Updates:

//Unoptimised Code
use ink_lang as ink;

#[ink::contract]
mod MyContract {
    #[ink(storage)]
    struct MyContract {
        value: u32,
    }

    #[ink(message)]
    fn update_value(&mut self, new_value: u32) {
        self.value = new_value;
        // Perform some other operations...
        self.value = new_value; // Redundant state update
    }
}
//Optimised Code
use ink_lang as ink;

#[ink::contract]
mod MyContract {
    #[ink(storage)]
    struct MyContract {
        value: u32,
    }

    #[ink(message)]
    fn update_value(&mut self, new_value: u32) {
        if self.value != new_value {
            self.value = new_value;
            // Perform some other operations...
        }
    }
}
//(2) Using View Functions for Read-Only Operations:

//Unoptimised Code
use ink_lang as ink;

#[ink::contract]
mod MyContract {
    #[ink(storage)]
    struct MyContract {
        value: u32,
    }

    #[ink(message)]
    fn get_value(&self) -> u32 {
        self.value
    }
}
//Optimised Code
use ink_lang as ink;

#[ink::contract]
mod MyContract {
    #[ink(storage)]
    struct MyContract {
        value: u32,
    }

    #[ink(message)]
    #[ink(view)] // Mark as a view function
    fn get_value(&self) -> u32 {
        self.value
    }
}


//(3) Use Constants Instead of Repeated Computations:

//Unoptimised Code
use ink_lang as ink;

#[ink::contract]
mod MyContract {
    #[ink(storage)]
    struct MyContract {
        #[ink(storage)]
        total: Balance,
    }

    #[ink(message)]
    fn calculate_fee(&self, amount: Balance) -> Balance {
        amount * self.get_fee_percentage() / 100
    }

    #[ink(message)]
    fn get_fee_percentage(&self) -> u32 {
        5
    }
}
//Optimised Code
use ink_lang as ink;

#[ink::contract]
mod MyContract {
    #[ink(storage)]
    struct MyContract {
        #[ink(storage)]
        total: Balance,
    }

    const FEE_PERCENTAGE: u32 = 5;

    #[ink(message)]
    fn calculate_fee(&self, amount: Balance) -> Balance {
        amount * FEE_PERCENTAGE / 100
    }
}



