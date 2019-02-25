use crate::mempool::Mempool;

pub fn run() {
    let max_weight = 4000000;
    let mempool = Mempool::new(max_weight);
    mempool.knapsack();
}
