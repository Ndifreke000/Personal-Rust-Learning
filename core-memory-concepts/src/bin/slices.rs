fn main() {
    println!("🔪 SLICES EXAMPLES 🔪");
    println!("{}", "=".repeat(40));
    
    string_slices();
    array_slices();
    slice_functions();
    text_analyzer_demo();
}

fn string_slices() {
    println!("\n--- String Slices ---");
    
    let s = String::from("hello world");
    
    // Different ways to create slices
    let hello = &s[0..5];  // or &s[..5]
    let world = &s[6..11]; // or &s[6..]
    let whole = &s[..];    // entire string
    
    println!("Original: {}", s);
    println!("hello: {}", hello);
    println!("world: {}", world);
    println!("whole: {}", whole);
    
    // String literals are slices
    let literal = "Hello, world!"; // This is &str
    println!("String literal: {}", literal);
    
    // Slices work with String and &str
    let first_word = find_first_word(&s);
    let first_word_literal = find_first_word(literal);
    
    println!("First word from String: {}", first_word);
    println!("First word from literal: {}", first_word_literal);
}

fn array_slices() {
    println!("\n--- Array Slices ---");
    
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4]; // [2, 3, 4]
    
    println!("Original array: {:?}", arr);
    println!("Slice [1..4]: {:?}", slice);
    
    // Different slice ranges
    let first_three = &arr[..3];
    let last_three = &arr[2..];
    let all = &arr[..];
    
    println!("First three: {:?}", first_three);
    println!("Last three: {:?}", last_three);
    println!("All elements: {:?}", all);
    
    // Working with slice functions
    let sum = sum_slice(&arr[1..4]);
    println!("Sum of slice [1..4]: {}", sum);
}

fn slice_functions() {
    println!("\n--- Functions with Slices ---");
    
    // Function that works with both String and &str
    let s1 = String::from("hello world");
    let s2 = "hello rust";
    
    println!("Processing String: {}", process_text(&s1));
    println!("Processing &str: {}", process_text(s2));
    
    // Slices prevent dangling references
    let word;
    {
        let sentence = String::from("hello rust programming");
        word = find_first_word(&sentence);
        println!("Word while sentence exists: {}", word);
    }
    // word is still valid here because it's a slice of the string literal part
    // But if sentence was dropped, this would be caught by the borrow checker
}

// Function that returns a string slice
fn find_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..] // Return entire string if no space found
}

fn process_text(text: &str) -> usize {
    text.split_whitespace().count()
}

fn sum_slice(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

fn text_analyzer_demo() {
    println!("\n--- Text Analyzer Demo ---");
    
    let text = "Rust is amazing for systems programming and web development";
    
    println!("Original text: {}", text);
    println!("Word count: {}", TextAnalyzer::count_words(text));
    
    let words = TextAnalyzer::get_words(text);
    println!("First 5 words: {:?}", &words[..5]);
    
    if let Some(found) = TextAnalyzer::find_substring(text, "amazing") {
        println!("Found substring: '{}'", found);
    }
    
    // Working with array slices in analysis
    let numbers = [10, 20, 30, 40, 50, 60, 70];
    let middle_slice = &numbers[2..5];
    println!("Middle slice: {:?}", middle_slice);
    println!("Average of middle slice: {:.1}", average_slice(middle_slice));
    
    // Demonstrate slice safety
    let safe_slice = safe_get_slice(&numbers, 1, 4);
    match safe_slice {
        Some(slice) => println!("Safe slice: {:?}", slice),
        None => println!("Invalid slice range"),
    }
}

struct TextAnalyzer;

impl TextAnalyzer {
    fn count_words(text: &str) -> usize {
        if text.trim().is_empty() {
            return 0;
        }
        text.split_whitespace().count()
    }
    
    fn get_words(text: &str) -> Vec<&str> {
        text.split_whitespace().collect()
    }
    
    fn find_substring<'a>(text: &'a str, pattern: &'a str) -> Option<&'a str> {
        match text.find(pattern) {
            Some(start) => {
                let end = start + pattern.len();
                Some(&text[start..end])
            }
            None => None,
        }
    }
}

fn average_slice(slice: &[i32]) -> f64 {
    if slice.is_empty() {
        return 0.0;
    }
    let sum: i32 = slice.iter().sum();
    sum as f64 / slice.len() as f64
}

fn safe_get_slice(arr: &[i32], start: usize, end: usize) -> Option<&[i32]> {
    if start <= end && end <= arr.len() {
        Some(&arr[start..end])
    } else {
        None
    }
}