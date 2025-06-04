fn main() {
    println!("🦀 Welcome to Core Memory Concepts in Rust! 🦀");
    println!("{}", "=".repeat(50));
    
    println!("\n📚 This project demonstrates:");
    println!("1️⃣  Ownership - Each value has exactly one owner");
    println!("2️⃣  Borrowing - Temporary access without ownership");  
    println!("3️⃣  References - Pointers guaranteed to be valid");
    println!("4️⃣  Slices - References to portions of data");
    
    println!("\n🚀 Run individual examples with:");
    println!("cargo run --bin ownership");
    println!("cargo run --bin borrowing");
    println!("cargo run --bin references");
    println!("cargo run --bin slices");
    println!("cargo run --bin complete_example");
    
    println!("\n🧪 Running a quick ownership demo:");
    demo_ownership();
}

// Quick demo to show ownership in action
fn demo_ownership() {
    println!("\n--- Ownership Demo ---");
    
    // String s1 owns the data "hello"
    let s1 = String::from("Hello, Rust!");
    println!("s1 owns: {}", s1);
    
    // Move: s1 gives ownership to s2
    let s2 = s1;
    println!("s2 now owns: {}", s2);
    
    // s1 can no longer be used here - ownership was moved!
    // println!("{}", s1); // This would cause a compile error
    
    // Pass ownership to function
    take_ownership(s2);
    println!("s2 was moved to function and is no longer accessible here");
}

fn take_ownership(some_string: String) {
    println!("Function received: {}", some_string);
    // some_string goes out of scope and is dropped here
}