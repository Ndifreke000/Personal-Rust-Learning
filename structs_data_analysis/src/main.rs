// structs_data_analysis/src/main.rs
//
// A minimal Web3 data-analysis project using structs. We model on-chain transactions,
// then compute simple metrics (total volume, unique addresses).

use std::collections::HashSet;

// 1. Define a struct to represent a blockchain transaction.
#[derive(Debug, Clone)]
struct TransactionRecord {
    tx_hash: String,
    from: String,
    to: String,
    value_wei: u128,
    timestamp: u64, // Unix epoch seconds
}

// 2. Implement methods on TransactionRecord as needed.
impl TransactionRecord {
    // Convert value from Wei to Ether (assuming 1 Ether = 10^18 Wei).
    fn value_in_ether(&self) -> f64 {
        (self.value_wei as f64) / 1e18
    }
}

// 3. Analytics struct groups a collection of TransactionRecords.
struct Analytics {
    records: Vec<TransactionRecord>,
}

impl Analytics {
    // Create a new Analytics instance from a vector of records.
    fn new(records: Vec<TransactionRecord>) -> Self {
        Analytics { records }
    }

    // Calculate total transferred volume (in Ether) across all records.
    fn total_volume_eth(&self) -> f64 {
        self.records
            .iter()
            .map(|r| r.value_in_ether())
            .sum()
    }

    // Count the number of unique addresses involved (from + to).
    fn unique_address_count(&self) -> usize {
        let mut set = HashSet::new();
        for r in &self.records {
            set.insert(r.from.clone());
            set.insert(r.to.clone());
        }
        set.len()
    }

    // Filter transactions above a threshold (in Wei) and return them.
    fn high_value_txs(&self, threshold_wei: u128) -> Vec<&TransactionRecord> {
        self.records
            .iter()
            .filter(|r| r.value_wei >= threshold_wei)
            .collect()
    }
}

fn main() {
    // 4. Sample data: a few dummy transaction records.
    let sample_data = vec![
        TransactionRecord {
            tx_hash: "0xabc123".into(),
            from: "0xAlice".into(),
            to: "0xBob".into(),
            value_wei: 5_000_000_000_000_000_000, // 5 ETH
            timestamp: 1_691_000_000,             // arbitrary
        },
        TransactionRecord {
            tx_hash: "0xdef456".into(),
            from: "0xCarol".into(),
            to: "0xDave".into(),
            value_wei: 1_200_000_000_000_000_000, // 1.2 ETH
            timestamp: 1_691_000_100,
        },
        TransactionRecord {
            tx_hash: "0xghi789".into(),
            from: "0xAlice".into(),
            to: "0xEve".into(),
            value_wei: 10_000_000_000_000_000_000, // 10 ETH
            timestamp: 1_691_000_200,
        },
    ];

    // 5. Initialize Analytics with sample data.
    let analytics = Analytics::new(sample_data);

    // 6. Compute and print total volume.
    let total_eth = analytics.total_volume_eth();
    println!("Total volume transferred: {:.3} ETH", total_eth);
    // Expected: 5 + 1.2 + 10 = 16.2 ETH

    // 7. Compute and print unique address count.
    let unique_count = analytics.unique_address_count();
    println!("Unique addresses involved: {}", unique_count);
    // Expected: {Alice, Bob, Carol, Dave, Eve} = 5

    // 8. Filter and display high-value transactions (≥ 5 ETH).
    let threshold = 5_000_000_000_000_000_000; // 5 ETH in Wei
    println!("\nHigh-value transactions (≥ 5 ETH):");
    for tx in analytics.high_value_txs(threshold) {
        println!(
            "- {}: {} → {} of {:.2} ETH at timestamp {}",
            tx.tx_hash,
            tx.from,
            tx.to,
            tx.value_in_ether(),
            tx.timestamp
        );
    }
}



