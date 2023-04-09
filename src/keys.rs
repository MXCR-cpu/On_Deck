use serde::{Serialize, Deserialize};
use ecies::PublicKey;
use ecies::SecretKey;

#[derive(Serialize, Deserialize)]
pub struct PlayerKeys {
    pub decryption_key: Vec<u8>, 
    pub encryption_key: Vec<u8>,
}

impl PlayerKeys {
    pub fn new((decryption_key, encryption_key): (SecretKey, PublicKey)) -> Self {
        Self {
            decryption_key: decryption_key.serialize().to_vec(),
            encryption_key: encryption_key.serialize().to_vec(),
        }
    }
}
