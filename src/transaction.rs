use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction{
    pub sender: String,
    pub reciever: String,
    pub amount: f64,
    pub signature: String,
}

impl Transaction{
    pub fn new(sender:String, reciever: String, amount: f64) -> Self{
        Transaction{
            sender,
            reciever,
            amount,
            signature: String::new(),
        }
    }

    pub fn coinbase(reciever: String, reward: f64) -> Self{
        Transaction{
            sender: "COINBASE".to_string(),
            reciever,
            amount: reward,
            signature: "COINBASE".to_string(),
        }
    }

    pub fn hash(&self) -> String{
        let data = format!("{}{}{}", self.sender, self.reciever, self.amount);

        let mut hasher = Sha256::new();
        Digest::update(&mut hasher, data.as_bytes());
        let result = hasher.finalize();

        hex::encode(result)
    }

    pub fn is_coinbase(&self) -> bool {
        self.sender == "COINBASE"
    }
}