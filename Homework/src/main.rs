const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT) * 5.0 / 9.0
}

//fn celsius_to_fahrenheit(c: f64) -> f64 {
  //  (c * 9.0 / 5.0) + FREEZING_POINT
//}

fn main() {
println!("\n--- Fahrenheit to Celsius ---");
    
let mut f_temp = 32.0;
    
    let c_temp = fahrenheit_to_celsius(f_temp);
    println!("{}°F is {:.2}°C", f_temp, c_temp);
    
    for _ in 0..5 {
        f_temp += 1.0;
        let c_temp = fahrenheit_to_celsius(f_temp);
        println!("{}°F is {:.2}°C", f_temp, c_temp);
    }
    
    println!("\n--- Number Analyzer ---");
    
    let numbers = [15, 8, 3, 22, 30, 7, 12, 45, 100, 1];
    
    println!("\nAnalyzing numbers:");
    for num in numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        }
        else if num % 3 == 0 {
            println!("{}: Fizz", num);
        }
        else if num % 5 == 0 {
            println!("{}: Buzz", num);
        }
        else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }
    
    println!("\nCalculating sum:");
    let mut index = 0;
    let mut sum = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);
    
    println!("\nFinding largest number:");
    let mut largest = numbers[0];
    let mut i = 1;
    loop {
        if i >= numbers.len() {
            break;
        }
        if numbers[i] > largest {
            largest = numbers[i];
        }
        i += 1;
    }
    println!("Largest number: {}", largest);
    
    println!("\n--- Guessing Game ---");
    
    let secret = 42;
    let guesses_array = [50, 30, 40, 45, 52, 39, 42];
    let mut guess_count = 0;
    let mut array_index = 0;
    
    loop {
        let guess = guesses_array[array_index];
        array_index += 1;
        guess_count += 1;
        
        let result = check_guess(guess, secret);
        
        if result == 0 {
            println!("Guess {}: {} - Correct!", guess_count, guess);
            break;
        } else if result == 1 {
            println!("Guess {}: {} - Too high!", guess_count, guess);
        } else {
            println!("Guess {}: {} - Too low!", guess_count, guess);
        }
    }
    
    println!("You guessed the number in {} attempts!", guess_count);
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}



