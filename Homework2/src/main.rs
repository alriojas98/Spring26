
fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    for o in (low..=high).step_by(step as usize) {
        *total += o;
    }
}


fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();

    let mut max_word = "";
    let mut max_count: usize = 0;

    for j in 0..words.len() {
        let mut count: usize = 0;
        for k in 0..words.len() {
            if words[j] == words[k] {
                count += 1;
            }
        }
        if count > max_count {
            max_count = count;
            max_word = words[j];
        }
    }

    (max_word.to_string(), max_count)
}

fn main() {
    // --- Assignment 1 ---
    let mut result = 0;
    sum_with_step(&mut result, 0, 1000, 1);
    println!("Sum 0 to 1000, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 20, 2);
    println!("Sum 0 to 20, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 100, 300, 3);
    println!("Sum 100 to 300, step 3: {}", result);

     // --- Assignment 2 ---
    let text = "I tried and tried to fix the code, but it just wouldn’t compile.";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
