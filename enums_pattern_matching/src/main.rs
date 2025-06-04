// enums_pattern_matching/src/main.rs
//
// A minimal Web3 transaction‐type classification project using enums and pattern matching.
// We define an enum of transaction kinds, then tally counts and volumes per kind.

use std::collections::HashMap;

// 1. Define an enum to represent the type of each transaction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TransactionType {
    Transfer,    // Simple ETH/ERC-20 transfer
    Swap,        // DEX swap
    Mint,        // Token mint event
    Burn,        // Token burn event
}

// 2. Define a struct that pairs type + core data.
#[derive(Debug, Clone)]
struct Event {
    tx_hash: String,
    tx_type: TransactionType,
    value_wei: u128,
}

// 3. Implement a helper to convert Wei → Ether.
impl Event {
    fn value_in_eth(&self) -> f64 {
        (self.value_wei as f64) / 1e18
    }
}

// 4. Function to process a list of events: count & sum per variant.
fn summarize_events(events: &[Event]) {
    // Initialize counters and accumulators.
    let mut counts: HashMap<TransactionType, usize> = HashMap::new();
    let mut volumes: HashMap<TransactionType, f64> = HashMap::new();

    for ev in events {
        // Pattern match on ev.tx_type to increment counters and volumes.
        match &ev.tx_type {
            TransactionType::Transfer => {
                *counts.entry(TransactionType::Transfer).or_default() += 1;
                *volumes.entry(TransactionType::Transfer).or_default() += ev.value_in_eth();
            }
            TransactionType::Swap => {
                *counts.entry(TransactionType::Swap).or_default() += 1;
                *volumes.entry(TransactionType::Swap).or_default() += ev.value_in_eth();
            }
            TransactionType::Mint => {
                *counts.entry(TransactionType::Mint).or_default() += 1;
                *volumes.entry(TransactionType::Mint).or_default() += ev.value_in_eth();
            }
            TransactionType::Burn => {
                *counts.entry(TransactionType::Burn).or_default() += 1;
                *volumes.entry(TransactionType::Burn).or_default() += ev.value_in_eth();
            }
        }
    }

    // Display summary.
    println!("Transaction Summary:");
    for tx_type in &[
        TransactionType::Transfer,
        TransactionType::Swap,
        TransactionType::Mint,
        TransactionType::Burn,
    ] {
        let count = counts.get(tx_type).copied().unwrap_or(0);
        let volume = volumes.get(tx_type).copied().unwrap_or(0.0);
        println!(
            "- {:?}: {} txs, {:.3} ETH total",
            tx_type, count, volume
        );
    }
}

// 5. Demonstrate `if let` for filtering a single variant.
fn print_only_swaps(events: &[Event]) {
    println!("\nListing all Swap transactions:");
    for ev in events {
        // Only print if tx_type == Swap, else skip.
        if let TransactionType::Swap = ev.tx_type {
            println!(
                "* {} of {:.4} ETH",
                ev.tx_hash,
                ev.value_in_eth()
            );
        }
    }
}

fn main() {
    // 6. Sample events with mixed types.
    let events = vec![
        Event {
            tx_hash: "0xaaa111".into(),
            tx_type: TransactionType::Transfer,
            value_wei: 2_000_000_000_000_000_000, // 2 ETH
        },
        Event {
            tx_hash: "0xbbb222".into(),
            tx_type: TransactionType::Swap,
            value_wei: 1_500_000_000_000_000_000, // 1.5 ETH
        },
        Event {
            tx_hash: "0xccc333".into(),
            tx_type: TransactionType::Mint,
            value_wei: 0,
        },
        Event {
            tx_hash: "0xddd444".into(),
            tx_type: TransactionType::Burn,
            value_wei: 0,
        },
        Event {
            tx_hash: "0xeee555".into(),
            tx_type: TransactionType::Swap,
            value_wei: 3_000_000_000_000_000_000, // 3 ETH
        },
        Event {
            tx_hash: "0xfff666".into(),
            tx_type: TransactionType::Transfer,
            value_wei: 0_500_000_000_000_000_000, // 0.5 ETH
        },
    ];

    // 7. Summarize all events by type.
    summarize_events(&events);

    // 8. Print only Swap transactions.
    print_only_swaps(&events);
}
