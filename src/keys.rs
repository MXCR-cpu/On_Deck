use ecies::encrypt;
use ecies::utils::generate_keypair;
use ecies::PublicKey;
use ecies::SecretKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerKeys {
    pub decryption_key: Vec<u8>,
    pub encryption_key: Vec<u8>,
    pub info_key: String,
}

impl PlayerKeys {
    pub fn new() -> Self {
        let (decryption_key, encryption_key): (SecretKey, PublicKey) = generate_keypair();
        let decryption_key: Vec<u8> = decryption_key.serialize().to_vec();
        let encryption_key: Vec<u8> = encryption_key.serialize().to_vec();
        let info_key: String = encrypt(&encryption_key, String::from("Request").as_bytes())
            .unwrap()
            .into_iter()
            .map(|element: u8| format!("{:x}", element))
            .collect::<Vec<String>>()
            .join("");

        Self {
            decryption_key,
            encryption_key,
            info_key,
        }
    }

    pub fn log(&self, player_id: &String) {
        println!("<<>> {} Created: {}", player_id, self);
    }

    pub fn public_key_string(&self) -> String {
        serde_json::to_string(&self.encryption_key).unwrap()
    }
}

impl std::fmt::Display for PlayerKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_buffer = |value: &Vec<u8>| {
            let mut index: usize = 0;
            value
                .iter()
                .map(|&item| {
                    index += 1;
                    format!(
                        "{:3}{}   ",
                        item,
                        if index % 5 == 0 { "\n\t\t" } else { "" }
                    )
                })
                .collect::<Vec<String>>()
                .join("")
        };
        write!(
            f,
            "{{\n\tencryption_key: [\n\n\t\t{}\n\n\t],\n\tdecryption_key: [\n\n\t\t{}\n\n\t]\n}}",
            line_buffer(&self.encryption_key),
            line_buffer(&self.decryption_key)
        )
    }
}

impl From<&PlayerKeys> for String {
    fn from(keys: &PlayerKeys) -> Self {
        serde_json::to_string(keys).unwrap_or("".to_string())
    }
}
