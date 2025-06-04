fn main() {
    println!("🔗 REFERENCES EXAMPLES 🔗");
    println!("{}", "=".repeat(40));
    
    basic_references();
    reference_rules();
    dangling_references();
    temperature_demo();
}

fn basic_references() {
    println!("\n--- Basic References ---");
    
    let x = 5;
    let y = &x; // y is a reference to x
    
    println!("x = {}", x);
    println!("y points to {}", *y); // Dereference with *
    println!("Address of x: {:p}", &x);
    println!("Value of y (address): {:p}", y);
    
    // References must always point to valid data
    let reference_to_five = &5; // Reference to literal
    println!("Reference to literal: {}", *reference_to_five);
}

fn reference_rules() {
    println!("\n--- Reference Rules ---");
    
    // Rule 1: References must always be valid
    let r;
    {
        let x = 5;
        r = &x; // This reference is valid within this scope
        println!("r points to: {}", *r);
    }
    // r would be invalid here if we tried to use it
    
    // Rule 2: References don't take ownership
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);
    println!("Length of '{}' is {}", s1, len); // s1 is still valid
    
    // Rule 3: Mutable references require mutable data
    let mut s2 = String::from("hello");
    let r1 = &mut s2;
    r1.push_str(" world");
    println!("Modified string: {}", r1);
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
    // s is a reference, so the String it points to is not dropped
}

fn dangling_references() {
    println!("\n--- Preventing Dangling References ---");
    
    // Rust prevents dangling references at compile time!
    // let reference_to_nothing = dangle(); // Won't compile!
    
    let valid_string = no_dangle();
    println!("Valid string: {}", valid_string);
    
    // Demonstrate lifetime relationships
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(&string1, string2);
    println!("The longest string is: {}", result);
}

// This would create a dangling reference - Rust prevents this!
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // We return a reference to s, but s is dropped!
}
*/

// Instead, return the owned value
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // Move ownership to caller
}

// Function with lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn temperature_demo() {
    println!("\n--- Temperature Converter Example ---");
    
    let mut temp = Temperature::new(25.0);
    
    // Immutable reference used internally by methods
    println!("{}°C = {}°F", temp.get_celsius(), temp.to_fahrenheit());
    
    // Mutable reference used internally
    temp.set_celsius(30.0);
    println!("Updated: {}°C = {}°F", temp.get_celsius(), temp.to_fahrenheit());
    
    // References in method calls
    let temp_ref = &temp;
    display_temperature(temp_ref);
    
    let temp_mut_ref = &mut temp;
    update_temperature(temp_mut_ref, 35.0);
    
    println!("Final: {}°C = {}°F", temp.get_celsius(), temp.to_fahrenheit());
}

struct Temperature {
    celsius: f64,
}

impl Temperature {
    fn new(celsius: f64) -> Self {
        Temperature { celsius }
    }
    
    // Method takes immutable reference to self
    fn to_fahrenheit(&self) -> f64 {
        (self.celsius * 9.0 / 5.0) + 32.0
    }
    
    fn get_celsius(&self) -> f64 {
        self.celsius
    }
    
    // Method takes mutable reference to self
    fn set_celsius(&mut self, new_temp: f64) {
        self.celsius = new_temp;
    }
}

fn display_temperature(temp: &Temperature) {
    println!("Temperature display: {}°C", temp.get_celsius());
}

fn update_temperature(temp: &mut Temperature, new_temp: f64) {
    temp.set_celsius(new_temp);
    println!("Temperature updated to: {}°C", temp.get_celsius());
}