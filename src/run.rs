use crate::matrix::Matrix;
use crate::mempool::Tx;

fn knapsack(mempool: &[Tx], max_weight: usize) {
    let n = mempool.len();
    let mut ks_mat = Matrix::new(vec![0; (n+1) * (max_weight+1)], n + 1, max_weight + 1);
    let mut keep_mat = Matrix::new(vec![0; (n+1) * (max_weight+1)], n + 1, max_weight + 1);

    // for (i, tx) in mempool.iter().enumerate().take(n).skip(1) {
    for i in 1..=n {
        let tx = &mempool[i-1];

        for w in 0..=max_weight {
            let pos_curr = ks_mat.to_position(i, w);

            // value[i] + ks_mat[i-1, w-w[i]]
            let pos_lhs = ks_mat.to_position(i.saturating_sub(1), w.saturating_sub(tx.weight as usize));
            let v_lhs = tx.value + ks_mat.data[pos_lhs];

            // ks_mat[i-1, w]
            let pos_rhs = ks_mat.to_position(i-1, w);
            let v_rhs = ks_mat.data[pos_rhs];

            // (w <= w[i]) && (value[i] + ks_mat[i-1, w-w[i]] > ks_mat[i-1, w])
            if (tx.weight <= w as u64) && (v_lhs > v_rhs) {

                ks_mat.data[pos_curr] = tx.value + ks_mat.data[pos_lhs];
                keep_mat.data[pos_curr] = 1;

            } else {
                ks_mat.data[pos_curr] = ks_mat.data[pos_rhs];
                keep_mat.data[pos_curr] = 0;
            }
        }
    }

    let keep_weight = max_weight;
    for i in (1..n).rev() {
        let curr_pos = keep_mat.to_position(i, keep_weight);
        if keep_mat.data[curr_pos] == 1 {
            println!("include this index: {} tx: {}", i, mempool[i].txid);
            keep_weight.saturating_sub(mempool[i].weight as usize);
        }
    }

    println!("ks_mat: {}", ks_mat);
    println!("keep_weight: {}", keep_weight);
    println!("keep_mat: {}", keep_mat);
}

pub fn run() {
    // Dummy tx's
    let tx0_parents = Some(vec![ String::from("0x123"), String::from("0x456")]);
    let tx0 = Tx::new(String::from("0x789"), 10, 5, tx0_parents);
    let tx1_parents = None;
    let tx1 = Tx::new(String::from("0x123"), 40, 4, tx1_parents);
    let tx2_parents = None;
    let tx2 = Tx::new(String::from("0x456"), 30, 6, tx2_parents);
    let tx3_parents = None;
    let tx3 = Tx::new(String::from("0xa73590"), 50, 3, tx3_parents);
   
    let max_weight = 10;
    let mempool = vec![tx0, tx1, tx2, tx3];

    knapsack(&mempool, max_weight);
}
