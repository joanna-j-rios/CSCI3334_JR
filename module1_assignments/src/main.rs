// -------------------------------- ASSIGNMENT 1 START ----------------------------------------

// Create program that converts temperature between Farenheit and Celcius

// 1. Declare constant for freezing point of water in Farenheit (32 degrees farenheit)
const FREEZE_FAHRENHEIT:f64 = 32.0; // declaring 64bit floating point constant ('const') variable

// 2. Implement two functions:
// fahrenheit_to_celsius(f: f64) -> f64: Converts Fahrenheit to Celsius
// celsius_to_fahrenheit(c: f64) -> f64: Converts Celsius to Fahrenheit

// function to convert farenheit to celcius
fn fahrenheit_to_celcius(f: f64) -> f64{
    // formula to convert from fahrenheit to celcius is : C = (F - 32) * 5/9 
    // where: C - Celcius | F - Fahrenheit
    
    // we are passed the fahrenheit value through 'f' in the parameter
    // we can use our constant in place of 32
    (f - FREEZE_FAHRENHEIT) * 5.0/9.0 // this (the conversion to celcius) is what will be returned
}

// function to convert celcius to fahrenheit
fn celcius_to_fahrenheit(c: f64) -> f64{
    // formula to convert from celcius to fahrenheit is : F = (C * 9/5) + 32 
    // where: C - Celcius | F - Fahrenheit
    (c * 9.0/5.0) + FREEZE_FAHRENHEIT // this (the conversion to fahrenheit) is what will be returned.
}

// -------------------------------- ASSIGNMENT 1 END ----------------------------------------



// -------------------------------- ASSIGNMENT 2 START ----------------------------------------

// Create a Rust program that analyzes a series of numbers. The program should:

// 1. Create an array of 10 integer numbers of your choice. --> "array with explicit type and size"
const NUMBERS: [i32;10] = [1,2,3,4,5,6,7,8,9,10];

// 2. Implement a function is_even(n: i32) -> bool that returns true if a number is even, false otherwise.
fn is_even(n: i32) -> bool{
    // even -> if there is a remainder of 0 when divided by 2
    // note: modulus returns remainder
    n % 2 == 0 // "if reaminder from number mod 2 is 0" --> will return true or false depending on if even or false
}

// -------------------------------- ASSIGNMENT 2 END ----------------------------------------



// -------------------------------- ASSIGNMENT 3 START ----------------------------------------

// Create a simple number guessing game in Rust. The program should:

// 1. Use a mutable variable to store a "secret" number (you can hard-code this).
const SECRET_NUM:i32 = 26; // variable for secret number--> decalring as constant since we can hard code this

// 2. Implement a function check_guess(guess: i32, secret: i32) -> i32 that returns:
fn check_guess(guess: i32, secret: i32) -> i32{
    //    0 if the guess is correct
    if guess == secret{ // if guess is correct i.e. if guess is equal to the secret number
        0 // returning 0 
    }
    //    1 if the guess is too high
    else if guess > secret{ //if guess is too high i.e. greater than the secret number
        1 // returning 1 
    }
    //   -1 if the guess is too low
    else{ // if guess is too low i.e. less than secret number --> this is last possible case
        -1 // returing 1
    }
}

// -------------------------------- ASSIGNMENT 3 END ----------------------------------------



fn main() {

    println!("");
    println!("----------------------- ASSIGNMENT 1 -------------------------------"); // ASSIGNMENT 1 START

    // CONVERSIONS FOR FAHRENHIET TO CELCIUS --> note only taking up to 1 decimal place for readability; hence use of :.1 to format
    // 3. In the main function:
    // a. Declare a mutable variable with a temperature in Fahrenheit
    let mut f_temp:f64 = 32.0;
    // b. Convert it to Celsius using your function and print the result
    let c_temp = fahrenheit_to_celcius(f_temp);
    println!("Starting temperature in fahrenhit is {}°F", f_temp);
    println!("Starting temperature converted to Celcius is: {:.1}°C", c_temp);
    // c. Use a loop to convert and print the next 5 integer temperatures 
    // (e.g., if you start with 32°F, print conversions for 33°F, 34°F, 35°F, 36°F, and 37°F)
    for i in 0..5 {
        f_temp += 1.0; // increment temp by 1 to get next temp to convert
        let c = fahrenheit_to_celcius(f_temp);
        println!("Conversion {} : {}°F to celcius is {:.1}°C", i + 1, f_temp, c);
    }

    println!("");
    // CONVERSIONS FOR CELCIUS TO FARENHEIT --> adding this because function celcius_to_fahrenheit was dead-code previously
    let mut c_temp:f64 = 10.0;
    let f_temp = celcius_to_fahrenheit(c_temp);
    println!("Starting temperature in celcius is {}°C", c_temp);
    println!("Starting temperature converted to Fahrenheit is: {:.1}°F", f_temp);
    for i in 0..5 {
        c_temp += 1.0; // increment temp by 1 to get next temp to convert
        let f = celcius_to_fahrenheit(c_temp);
        println!("Conversion {} : {}°C to farenheit is {:.1}°F", i + 1, c_temp, f);
    }



    println!("");
    println!("----------------------- ASSIGNMENT 2 -------------------------------"); // ASSIGNMENT 2 START

    // 3. Use a for loop to iterate through the array and for each number:
    for number in NUMBERS.iter(){
        //let mut result = String::new(); // creating new empty string to store result of each number
        //    If the number is divisible by 3, print "Fizz" instead
        if number % 3 == 0{
            //result.push_str("Fizz");
            println!("Number: {} | Result: Fizz", number); // printing number and their result
        }
        //    If the number is divisible by 5, print "Buzz" instead
        else if number % 5 == 0{
            //result.push_str("Buzz");
            println!("Number: {} | Result: Buzz", number); // printing each number and their result

        }
        //    If it's divisible by both 3 and 5, print "FizzBuzz"
        else if number % 3 == 0 && number % 5 == 0{
            //result.push_str("FizzBuzz");
            println!("Number: {} | Result: FizzBuzz", number); // printing each number and their result

        }
        //    Print whether it's even or odd using your is_even function
        else {
            if is_even(*number) { // if even
                //result.push_str("Even");
                println!("Number: {} | Result: Even", number); // printing each number and their result

            }
            else if !is_even(*number){ // if odd
               //result.push_str("Odd");
                println!("Number: {} | Result: Odd", number); // printing each number and their result

            }
        }
        //println!("Number: {} | Result: {}", number, result); // printing each number and their result
    }

    println!("");
    // 4. Use a while loop to find and print the sum of all numbers in the array.
    let mut sum:i32 = 0; // variable that will keep sum of array numbers as we're traversing through
    let mut index:usize = 0; // counter to help us access each element
    while index < NUMBERS.len(){ // traverse through array from index 0 to end
        sum += NUMBERS[index]; // add current number to sum
        index += 1; // increment index to move on to next array element
    }
    println!("The sum of all numbers in the array is: {}", sum);

    println!("");
    // 5. Use a loop to find and print the largest number in the array.
    let mut largest_num = NUMBERS[0]; // assume largest value is first one for now
    let mut index:usize = 0; // again using counter to keep track of position in array so we dont go out of bounds
    loop{ // inifinite loop that will run until it hits a break statements - which in our case will be once theres no more numbers to compare (i.e. reached end of array --> so current largest is largest possible number)
        if index >= NUMBERS.len(){ // if end of array is reached, stop (i.e. stop once index reaches or surpases length i.e. no longer less than)
            break; // exits loop
        }

        let current_num = NUMBERS[index]; // variable to hold current number
        if current_num > largest_num { // compare current number to (current) largest number, if its greater this our (new) largest
            largest_num = current_num; // update largest number
        }

        index += 1; // update index to move to next element/number in array
    }
    println!("The largest number in the array is: {}", largest_num);


    
    println!("");
    println!("----------------------- ASSIGNMENT 3 -------------------------------"); // ASSIGNMENT 3 START

    // 3. In the main function:
    let mut num_guesses:i32 = 0; // decalring/initializing variable that keeps track of number of guesses -> starting at 0 as game has not begun
    let mut guess:i32;  // variable to hold user guesses
    println!("Let's play a guessing game. Try and guess the secret number.");

    //    Use a loop to repeatedly:
    loop{
        num_guesses += 1; // increment guess counter
        //    Set a mutable guess variable to a number of your choice (simulating user input)
        if num_guesses == 1{ // first guess: too low
            guess = 6;
        }
        else if num_guesses == 2{ // second guess: too high
            guess = 56;
        }
        else if num_guesses == 3{ // third guess: too high
            guess = 46;
        }
        else if num_guesses == 4{ // third guess: too high
            guess = 36;
        }
        else if num_guesses == 5{ // fourth guess: correct
            guess = 26;
        }
        else { // just in case it doesnt break so we dont have an infinite loop
            break;
        }

        //    Call the check_guess function
        let game_result = check_guess(guess, SECRET_NUM);

        //    Use an if-else expression to print whether the guess was correct, too high, or too low
        //    if the guess was correct, break the loop     
        if game_result == 0{
            println!("Your guess was {}. Correct! You guessed the secret number.", guess);
            break;
        }
        else if game_result == 1{ 
            println!("Your guess was {}. Incorrect! Too high. Try again.", guess);
        }
        else {
            println!("Your guess was {}. Incorrect! Too low. Try again", guess);
        }
        
    }

    //    After the loop ends, print how many guesses it took (you'll need to track this in a variable)
    println!("Game Over. The secret number was {}. You got it in {} guesses", SECRET_NUM, num_guesses);
    println!("");

}
