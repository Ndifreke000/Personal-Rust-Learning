use crate::inventory::Inventory;

pub fn format_report(inventory: &Inventory) -> String {
    let mut report = format!("Inventory Report for {}\n", inventory.event_name());
    report.push_str("----------------------------\n");

    for item in inventory.list_items() {
        report.push_str(&format!("{}: {}\n", item.name, item.quantity));
    }

    report
}
