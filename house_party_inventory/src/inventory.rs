// inventory.rs

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub quantity: u32,
}

pub struct Inventory {
    event: String,
    items: Vec<Item>,
}

impl Inventory {
    pub fn new(event: &str) -> Self {
        Inventory {
            event: event.to_string(),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, name: &str, quantity: u32) {
        self.items.push(Item {
            name: name.to_string(),
            quantity,
        });
    }

    pub fn list_items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn event_name(&self) -> &str {
        &self.event
    }
}
