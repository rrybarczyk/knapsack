use crate::mempool::{Mempool, Tx};

pub fn run() {
    // Dummy tx's
    let tx0_parents = Some(vec![String::from("0x123"), String::from("0x456")]);
    let tx0 = Tx::new(String::from("0x789"), 10, 5, tx0_parents);
    let tx1_parents = None;
    let tx1 = Tx::new(String::from("0x123"), 40, 4, tx1_parents);
    let tx2_parents = None;
    let tx2 = Tx::new(String::from("0x456"), 30, 6, tx2_parents);
    let tx3_parents = None;
    let tx3 = Tx::new(String::from("0xa73590"), 50, 3, tx3_parents);

    let max_weight = 10;
    let mempool = Mempool::new(vec![tx0, tx1, tx2, tx3], max_weight);
    mempool.knapsack();
}
