fn main() {
    let mut owner = String::from("Hello, world!"); // owner owns the String data
    
    let borrower = &my_string; // borrower borrows owner's data with an immutable reference
    println!("The borrowed string is: {}", borrower); // use the borrowed data
    
    let owned_string = give_ownership(owner); // transfer ownership of owner's data to owned_string
    println!("The owned string is: {}", owned_string); // use the owned data
    
    // println!("The borrowed string is: {}", borrower); // ERROR: cannot use borrower here, since owner was moved and its ownership was transferred to owned_string
}

fn give_ownership(some_string: String) -> String { // takes ownership of some_string
    some_string // return some_string, which transfers its ownership to the caller
}
