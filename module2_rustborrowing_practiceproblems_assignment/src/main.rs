// Assignment Instructions:
// 1. Implement the functions for each problem.
// 2. Make sure your code compiles without errors.
// 3. Test your implementations with the provided main functions tests
// * Remember to use Rust's borrowing rules correctly and think about how ownership is managed in each scenario.


// Problem #1: String Concatenation with Borrowing
// Write a function that concatenates two strings without taking ownership, i.e., by borrowing.

fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here

    let mut result = s1.clone();
    result.push_str(s2);
    result
}


// Problem #2: Clone and Modify
// Given a string, clone it and modify the cloned string by appending a new word. Print both the original string and the cloned, modified string to show that the original has not been changed.

fn clone_and_modify(s: &String) -> String {
    // Your code here

    let mut modified = s.clone();
    modified.push_str("World!");
    modified
}


// Problem #3: Mutable Reference Sum
// Write a modified sum function that takes a mutable reference for the destination of the sum from low to high.

#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    // 0

    let mut current = low; // current starts at low... in this case 0
    while current <= high { // iterate from 0 to 100
        *total += current; // explicitly dereferencing total to get actual value since its a reference
        current += 1; // this is how we'll move from 0 to 100 --> incrementing by 1 from 0 to 100 (<=)
    }

}

fn main() {

    // Problem 1
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"

    // Problem 2
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"

    // Problem 3
    // create necessary variables and test your function for low 0 high 100
    // total should be 5050
    let mut total = 0; // total is going to be referrenced for the destination of the sum from low to high
    let low = 0;
    let high = 100;
    sum(&mut total, low, high); // passing referrence to total --> pointer to total which holds value 0 at start. will update by end of call
    println!("The Sum from {} to {} is {}", low, high, total);


}


