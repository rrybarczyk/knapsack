#[derive(Debug)]
pub struct Tx {
    pub txid: String,
    pub value: u64,
    pub weight: u64,
    pub parent: Option<Vec<String>>,
}

impl Tx {
    pub fn new(txid:String ,value: u64, weight: u64, parent: Option<Vec<String>>) -> Tx {
        Tx { txid, value, weight, parent }
    }
}

