fn main() {
    println!("📚 BORROWING EXAMPLES 📚");
    println!("{}", "=".repeat(40));
    
    immutable_borrowing();
    mutable_borrowing();
    borrowing_rules();
    library_demo();
}

fn immutable_borrowing() {
    println!("\n--- Immutable Borrowing ---");
    
    let s = String::from("hello world");
    
    // Immutable borrowing - we can read but not modify
    let len = calculate_length(&s);
    println!("Length of '{}' is {}", s, len);
    
    // We can have multiple immutable references
    let r1 = &s;
    let r2 = &s;
    println!("r1: {} and r2: {}", r1, r2);
    
    // Original owner can still be used
    println!("Original string: {}", s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s goes out of scope but doesn't drop the value (it's borrowed)
}

fn mutable_borrowing() {
    println!("\n--- Mutable Borrowing ---");
    
    let mut s = String::from("hello");
    println!("Before: {}", s);
    
    change_string(&mut s);
    println!("After: {}", s);
    
    // Can't have mutable and immutable references at same time
    let r1 = &s; // immutable borrow
    // let r2 = &mut s; // This would error!
    println!("Immutable reference: {}", r1);
}

fn change_string(s: &mut String) {
    s.push_str(", world!");
}

fn borrowing_rules() {
    println!("\n--- Borrowing Rules ---");
    
    let mut s = String::from("hello");
    
    // Rule 1: Multiple immutable references are OK
    {
        let r1 = &s;
        let r2 = &s;
        println!("Multiple immutable refs: {} and {}", r1, r2);
        // r1 and r2 go out of scope here
    }
    
    // Rule 2: Only one mutable reference at a time
    {
        let r1 = &mut s;
        r1.push_str(" world");
        println!("Mutable ref: {}", r1);
        // r1 goes out of scope here
    }
    
    // Rule 3: Can't mix mutable and immutable references
    // This demonstrates the lifetime concept
    let r1 = &s; // immutable borrow starts
    let r2 = &s; // another immutable borrow
    println!("Immutable refs: {} and {}", r1, r2);
    // immutable borrows end here (last use)
    
    let r3 = &mut s; // mutable borrow starts
    r3.push('!');
    println!("Mutable ref: {}", r3);
    // mutable borrow ends here
}

fn library_demo() {
    println!("\n--- Library Book Example ---");
    
    let mut book = Book {
        title: String::from("Rust Programming"),
        pages: 200,
        available: true,
    };
    
    // Multiple people can read the book info (immutable borrows)
    read_book_info(&book);
    check_availability(&book);
    
    // Only one person can update the book at a time (mutable borrow)
    update_book(&mut book, 250);
    
    // Can read again after mutable borrow ends
    read_book_info(&book);
}

struct Book {
    title: String,
    pages: u32,
    available: bool,
}

fn read_book_info(book: &Book) {
    println!("Reading info for '{}'", book.title);
    println!("  Pages: {}", book.pages);
}

fn check_availability(book: &Book) -> bool {
    println!("  Available: {}", book.available);
    book.available
}

fn update_book(book: &mut Book, new_pages: u32) {
    book.pages = new_pages;
    book.available = false; // Someone is updating it
    println!("Updated '{}' to {} pages", book.title, book.pages);
    book.available = true; // Available again
}