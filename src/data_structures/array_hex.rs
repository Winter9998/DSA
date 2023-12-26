// Import the `rand` crate for random number generation
use rand::Rng;

/// Generates a random number within a specified range
/// 
/// # Returns
/// A random number of type `u64` within the range of `mod` to `u64::MAX`
pub fn generate_number() -> u64 {
    // Define the maximum possible value for a `u64`
    let max = u64::MAX;
    // Calculate the modulus to create a range starting point
    let r#mod = u64::MAX % 2_u64.pow(20);
    // Generate a random number within the range of `mod` to `max`
    let number = rand::thread_rng().gen_range(r#mod..=max);
    number
}

/// Creates two vectors, one containing random `u64` numbers, and another with pointers to `i32`
/// 
/// # Notes
/// The function populates the vectors with random data and prints their contents
pub fn make_array_hex() {
    // Vector to store pointers to `i32`
    let mut vec: Vec<*const i32> = Vec::new();
    // Vector to store random `u64` numbers
    let mut avec: Vec<u64> = Vec::new();
    // Counter variable for the loop
    let mut i: u64 = 0;

    let arr = [64, 58, 51484, 4484, 97941, 1, 574];

    // Loop to populate the vectors
    loop {
        // Add a random `u64` number to `avec`
        avec.push(generate_number());
        // Generate a random number and convert it to a pointer to `i32`, then add to `vec`
        let hex = generate_number() as *const i32;
        vec.push(hex);
        // Increment the counter
        i += 1;
        // Exit the loop after 8 iterations
        if i == 8 {
            break;
        }
    }

    // Print the memory addresses stored in `vec`
    for ptr in &vec {
        println!("{:p}", ptr);
    }
    // Print the random numbers stored in `avec`
    println!("{:?}", avec);

    // print second index of the array
    println!("{}", arr[2]);
}
