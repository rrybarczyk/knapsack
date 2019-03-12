use crate::mempool::Mempool;

pub fn run() {
    // read csv
    // parse tx -> vec<Tx>
    // knapsack(&Vec<Tx>, max_weight) -> Result<Vec<txids>, Box<Error>>
    // write_result
    // type that is the state i need for each traversal
    // ajd list, parent list, current position: KnapsackContext: stores transiet state we use while
    // doing problem
    // write_tx shold be in disply
    let max_weight = 4000000;
    let mempool = Mempool::new(max_weight);
    mempool.knapsack();
}
