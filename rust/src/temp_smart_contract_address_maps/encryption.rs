use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};

pub fn encrypt(secret_key: &[u8], data: &String) -> Vec<u8> {
    let key = Key::from_slice(secret_key); // 32-bytes
    let cipher = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(b"unique nonce"); // 12-bytes; unique per message
    let ciphertext = cipher.encrypt(nonce, data.as_ref())
        .expect("encryption failure!");  // NOTE: handle this error to avoid panics!
    ciphertext
}

fn decrypt(secret_key: &[u8], ciphertext: Vec<u8>) -> Vec<u8> {
    let key = Key::from_slice(secret_key); // 32-bytes
    let cipher = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(b"unique nonce"); // 12-bytes; unique per message

    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!");  // NOTE: handle this error to avoid panics!
    plaintext
}

#[cfg(test)]
mod data_encryption {
    use std::str::from_utf8;
    use crate::temp_smart_contract_address_maps::crud_paragraph::BasicParagraph;
    use crate::temp_smart_contract_address_maps::encryption::{decrypt, encrypt};

    #[test]
    fn encryption() {
        let data = BasicParagraph {
            title: String::from("title"),
            body: String::from("body"),
        };
        let serialized_data = serde_json::to_string(&data).unwrap();
        let secret_key = "my very secret key 123 abcdefghi".as_bytes();

        let encrypted_data = encrypt(secret_key, &serialized_data);

        let decrypted_data = decrypt(secret_key, encrypted_data);

        assert_eq!(&decrypted_data, &serialized_data.as_bytes());

        let deserialized_data = serde_json::from_slice::<BasicParagraph>(&decrypted_data).unwrap();
        println!("{:?}, {:?}", data, deserialized_data);
        println!("{}", from_utf8(&decrypted_data.as_slice()).unwrap());
    }
}