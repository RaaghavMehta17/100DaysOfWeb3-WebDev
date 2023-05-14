pragma solidity ^0.8.0;

// Define a new contract called MemoryExample
contract MemoryExample {

    // Define a new struct called Person
    struct Person {
        string name; // A string field called 'name'
        uint age; // An unsigned integer field called 'age'
    }

    // Define a new function called createPerson that returns a Person struct
    function createPerson(string memory _name, uint _age) public returns (Person memory) {
        // Create a new Person struct called newPerson with the given name and age
        Person memory newPerson = Person(_name, _age);
        // Return the newPerson struct
        return newPerson;
    }
}
