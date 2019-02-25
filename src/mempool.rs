use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use crate::matrix::Matrix;

#[derive(Debug, Deserialize)]
pub struct Tx {
    pub txid: String,
    pub value: u64,
    pub weight: u64,
    pub parent: Option<Vec<String>>,
}

pub struct Mempool {
    pub mempool: Vec<Tx>,
    pub max_weight: usize,
}

impl Mempool {
    pub fn new(max_weight: usize) -> Mempool {
        let mempool_vec = Mempool::read_csv();
        Mempool {
            mempool: mempool_vec.unwrap(),
            max_weight: max_weight,
        }
    }

    fn read_csv() -> Result<Vec<Tx>, Box<Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(io::stdin());
        let mut mempool_vec = Vec::new();
        for result in rdr.records() {
            let record = result?;
            let row: Tx = record.deserialize(None)?;

            mempool_vec.push(row);
        }

        Ok(mempool_vec)
    }

    fn clear_result() {
        if Path::new("./tx_result.txt").exists() {
            fs::remove_file("tx_result.txt").unwrap();
        }
    }

    fn write_tx(selected_tx: &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("./tx_result.txt")
            .unwrap();

        if let Err(e) = writeln!(file, "{}", selected_tx) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    pub fn knapsack(&self) {
        let n = self.mempool.len();
        let max_weight = self.max_weight;
        let mut ks_mat = Matrix::new(vec![0; (n + 1) * (max_weight + 1)], n + 1, max_weight + 1);
        let mut keep_mat = Matrix::new(vec![0; (n + 1) * (max_weight + 1)], n + 1, max_weight + 1);

        for i in 1..=n {
            let tx = &self.mempool[i - 1];

            for w in 0..=max_weight {
                let pos_curr = ks_mat.to_position(i, w);

                // value[i] + ks_mat[i-1, w-w[i]]
                let pos_lhs =
                    ks_mat.to_position(i.saturating_sub(1), w.saturating_sub(tx.weight as usize));
                let v_lhs = tx.value + ks_mat.data[pos_lhs];

                // ks_mat[i-1, w]
                let pos_rhs = ks_mat.to_position(i - 1, w);
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

        Mempool::clear_result();
        let mut keep_weight = max_weight;
        for i in (1..=n).rev() {
            let curr_pos = keep_mat.to_position(i, keep_weight);
            if keep_mat.data[curr_pos] == 1 {
                Mempool::write_tx(&self.mempool[i - 1].txid);
                keep_weight = keep_weight.saturating_sub(self.mempool[i - 1].weight as usize);
            }
        }
    }
}
