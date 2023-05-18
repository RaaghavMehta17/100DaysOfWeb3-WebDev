//(1)Minimize Storage Reads and Writes:

// Inefficient storage usage
contract StorageExample {
    uint public counter;

    function increment() public {
        counter++;
    }
}
// Optimized storage usage
contract StorageExample {
    uint public counter;

    function increment() public {
        uint temp = counter;
        temp++;
        counter = temp;
    }
}


//(2)Use Memory instead of Storage:
// Inefficient storage usage
function calculateSum(uint[] memory numbers) public pure returns (uint) {
    uint sum = 0;
    for (uint i = 0; i < numbers.length; i++) {
        sum += numbers[i];
    }
    return sum;
}

// Optimized memory usage
function calculateSum(uint[] memory numbers) public pure returns (uint) {
    uint sum = 0;
    uint len = numbers.length;
    for (uint i = 0; i < len; i++) {
        sum += numbers[i];
    }
    return sum;
}

//(3)Use View and Pure Functions:
// Inefficient state-modifying function
function multiply(uint a, uint b) public returns (uint) {
    return a * b;
}

// Optimized stateless function
function multiply(uint a, uint b) public pure returns (uint) {
    return a * b;
}



