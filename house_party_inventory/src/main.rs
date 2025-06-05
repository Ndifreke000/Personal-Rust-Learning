mod inventory;
mod utils;

use inventory::Inventory;
use utils::formatter::format_report;

fn main() {
    let mut party_inventory = Inventory::new("House Party 7");

    party_inventory.add_item("Chips", 10);
    party_inventory.add_item("Drinks", 25);
    party_inventory.add_item("Cups", 50);
    party_inventory.add_item("Speakers", 1);

    let report = format_report(&party_inventory);
    println!("{}", report);
}
