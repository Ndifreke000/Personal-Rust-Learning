fn main() {
    println!("🛒 COMPLETE EXAMPLE: Shopping List Manager 🛒");
    println!("{}", "=".repeat(50));
    
    shopping_list_demo();
    string_slice_demo();
    parameter_types_demo();
}

fn shopping_list_demo() {
    println!("\n--- Shopping List Manager ---");
    
    // Create initial items (ownership transferred to vector)
    let initial_items = vec![
        String::from("milk"),
        String::from("bread"),
        String::from("eggs"),
    ];
    
    // Create shopping list (takes ownership of vector)
    let mut list = ShoppingList::new(initial_items);
    
    // Borrow immutably to display
    list.display();
    
    // Borrow mutably to add items
    list.add_item(String::from("cheese"));
    list.add_item(String::from("apples"));
    
    println!("\nAfter adding items:");
    list.display();
    
    // Get a slice of all items
    let all_items = list.get_items();
    println!("\nItems as slice: {:?}", all_items);
    
    // Search for an item (borrowing)
    match list.find_item("milk") {
        Some(item) => println!("\nFound item: {}", item),
        None => println!("\nItem not found"),
    }
    
    // Remove an item (takes ownership from list)
    if let Some(removed) = list.remove_item(0) {
        println!("\nRemoved: {}", removed);
        // 'removed' owns the string now
    }
    
    println!("\nFinal list:");
    list.display();
    
    // Demonstrate item statistics
    let stats = list.get_statistics();
    println!("\nList Statistics:");
    println!("  Total items: {}", stats.total_items);
    println!("  Average length: {:.1}", stats.average_length);
    println!("  Longest item: {}", stats.longest_item);
}

fn string_slice_demo() {
    println!("\n--- String Slice Demo ---");
    
    let shopping_note = String::from("Buy: milk, bread, eggs, cheese");
    
    // Get slice of the list part
    let list_part = &shopping_note[5..]; // "milk, bread, eggs, cheese"
    println!("List part: {}", list_part);
    
    // Split into individual items (returns iterator of &str)
    let items: Vec<&str> = list_part.split(", ").collect();
    println!("Individual items: {:?}", items);
    
    // Find first and last items
    if let Some(first_item) = items.first() {
        println!("First item: {}", first_item);
    }
    if let Some(last_item) = items.last() {
        println!("Last item: {}", last_item);
    }
    
    // Work with item slices
    if items.len() >= 2 {
        let middle_items = &items[1..items.len()-1];
        println!("Middle items: {:?}", middle_items);
    }
}

fn parameter_types_demo() {
    println!("\n--- Parameter Types Demo ---");
    
    let owned_string = String::from("I own this");
    let string_slice = "I'm a slice";
    let items = vec![String::from("item1"), String::from("item2")];
    
    // Different ways to pass strings and slices
    process_shopping_data(
        owned_string,           // Takes ownership
        &items[0],             // Borrows String
        string_slice,          // String slice
        &items,                // Slice of Strings
    );
    
    // owned_string is no longer valid here
    // But we can still use string_slice and items
    println!("Remaining items: {:?}", items);
    println!("String slice still valid: {}", string_slice);
}

// Complete example demonstrating all concepts
struct ShoppingList {
    items: Vec<String>,
}

impl ShoppingList {
    // Takes ownership of the vector
    fn new(items: Vec<String>) -> Self {
        ShoppingList { items }
    }
    
    // Borrows self immutably
    fn display(&self) {
        println!("Shopping List:");
        for (i, item) in self.items.iter().enumerate() {
            // item is a reference to String (&String)
            println!("{}. {}", i + 1, item);
        }
    }
    
    // Borrows self mutably
    fn add_item(&mut self, item: String) {
        self.items.push(item); // Takes ownership of item
    }
    
    // Returns a slice of the items
    fn get_items(&self) -> &[String] {
        &self.items
    }
    
    // Borrows self immutably, takes string slice
    fn find_item(&self, search: &str) -> Option<&String> {
        self.items.iter().find(|&item| item.contains(search))
    }
    
    // Returns owned String (not borrowing)
    fn remove_item(&mut self, index: usize) -> Option<String> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }
    
    // Returns statistics about the list
    fn get_statistics(&self) -> ListStatistics {
        if self.items.is_empty() {
            return ListStatistics {
                total_items: 0,
                average_length: 0.0,
                longest_item: String::new(),
            };
        }
        
        let total_items = self.items.len();
        let total_length: usize = self.items.iter().map(|item| item.len()).sum();
        let average_length = total_length as f64 / total_items as f64;
        
        let longest_item = self.items
            .iter()
            .max_by_key(|item| item.len())
            .unwrap()
            .clone();
        
        ListStatistics {
            total_items,
            average_length,
            longest_item,
        }
    }
}

struct ListStatistics {
    total_items: usize,
    average_length: f64,
    longest_item: String,
}

// Helper function showing different parameter types
fn process_shopping_data(
    owned_string: String,      // Takes ownership
    borrowed_string: &String,  // Borrows String
    string_slice: &str,        // String slice (most flexible)
    item_slice: &[String],     // Slice of Strings
) {
    println!("Processing different parameter types:");
    println!("  Owned: {}", owned_string);
    println!("  Borrowed: {}", borrowed_string);
    println!("  Slice: {}", string_slice);
    println!("  Items: {} items total", item_slice.len());
    
    // owned_string is dropped here
    // Other parameters are just borrowed
    
    // Show first item from slice if available
    if let Some(first) = item_slice.first() {
        println!("  First item from slice: {}", first);
    }
}