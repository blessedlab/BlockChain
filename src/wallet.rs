use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use crate::transaction::Transaction;

pub struct Wallet {
    signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
    pub address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let address = Self::compute_address(&verifying_key);

        Wallet{
            signing_key,
            verifying_key,
            address,
        }
    }

    fn compute_address(verifying_key: &VerifyingKey) -> String{
        let mut hasher = Sha256::new();
        Digest::update(&mut hasher, verifying_key.as_bytes());
        let hash = hasher.finalize();

        hex::encode(&hash[..20])
    }

    pub fn public_key_hex(&self) -> String {
        hex::encode(self.verifying_key.as_bytes())
    }

    pub fn sign_transaction(&self, tx: &Transaction) -> Transaction{
        let tx_hash = tx.hash();
        let signature = self.signing_key.sign(tx_hash.as_bytes());
        let mut signed_tx = tx.clone();
        signed_tx.signature = hex::encode(signature.to_bytes());

        signed_tx
    }

    pub fn verify_transaction(tx: Transaction, public_key_hex: &str) -> bool{
        if tx.is_coinbase(){
            return true;
        }

        let pub_key_bytes = match hex::decode(public_key_hex){
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        let pub_key_array: [u8; 32] = match pub_key_bytes.try_into(){
            Ok(arr) => arr,
            Err(_) => return false,
        };

        let verifying_key = match VerifyingKey::from_bytes(&pub_key_array) {
            Ok(key) => key,
            Err(_) => return false,
        };

        let sig_bytes = match hex::decode(&tx.signature) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        let sig_array: [u8; 64] = match sig_bytes.try_into() {
            Ok(arr) => arr,
            Err(_) => return false,
        };

        let signature = ed25519_dalek::Signature::from_bytes(&sig_array);

        let tx_hash = tx.hash();
        verifying_key.verify(tx_hash.as_bytes(), &signature).is_ok()
    }
}