use crate::objects::EncryptionLevel;
use blake2::{Blake2b512, Digest};
use openssl::symm::Cipher;
use rand::{Rng, SeedableRng};
use std::io::{BufRead, Read, Write};

impl EncryptionLevel {
    fn key_iv_for_cipher(cipher: &Cipher, password: &str) -> (Vec<u8>, Vec<u8>) {
        fn string_to_seed(seed: &str) -> [u8; 32] {
            let mut hasher = Blake2b512::new();
            Digest::update(&mut hasher, seed.as_bytes());
            let hash_result = hasher.finalize();

            let mut seed_bytes = [0u8; 32];
            seed_bytes.copy_from_slice(&hash_result[..32]);
            seed_bytes
        }

        fn generate_random_vec(seed: &str, length: usize) -> Vec<u8> {
            let seed_hash = string_to_seed(seed); // Convert string to a reproducible seed
            let mut rng = rand_hc::Hc128Rng::from_seed(seed_hash); // Seed the PRNG with the hashed seed

            let some_number: Option<u64> = Some(2);

            match some_number {
                None => {}
                Some(number) => {
                    number * 3;
                }
            }

            // Generate the random Vec<u8>
            (0..length).map(|_| rng.gen()).collect()
        }

        let key_len = cipher.key_len();
        let iv_len = cipher.iv_len().unwrap_or(0);

        let key = generate_random_vec(password, key_len);
        let iv = generate_random_vec(password, iv_len);

        (key, iv)
    }

    pub fn to_encoder<'b, W: Write + 'b>(&self, w: W) -> Box<dyn Write + 'b> {
        match self.clone() {
            EncryptionLevel::None => Box::new(w),
            EncryptionLevel::Symmetrical { password } => {
                let cipher = Cipher::aes_256_cbc();
                let (key, iv) = EncryptionLevel::key_iv_for_cipher(&cipher, password.insecure());
                Box::new(cryptostream::write::Encryptor::new(w, cipher, &key, &iv).unwrap())
            }
        }
    }

    pub fn to_decoder<R: Read + 'static>(&self, r: R) -> Box<dyn Read> {
        match self.clone() {
            EncryptionLevel::None => Box::new(r),
            EncryptionLevel::Symmetrical { password } => {
                let cipher = Cipher::aes_256_cbc();
                let (key, iv) = EncryptionLevel::key_iv_for_cipher(&cipher, password.insecure());
                Box::new(cryptostream::read::Decryptor::new(r, cipher, &key, &iv).unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::channels::ChannelWriter;
    use std::io::Cursor;
    use std::sync::mpsc;

    #[test]
    fn test_cipher() {
        let cipher = Cipher::aes_256_cbc();
        let (key, iv) = EncryptionLevel::key_iv_for_cipher(&cipher, "password");
        assert_eq!(key.len(), 32);
        assert_eq!(iv.len(), 16);
    }

    #[test]
    fn test_information_preserve() {
        let encryption = EncryptionLevel::Symmetrical {
            password: "password".into(),
        };

        let input = [1, 2, 4, 8, 16, 32, 64, 128, 255];
        let (tx, rx) = mpsc::channel();
        let mut encoder = encryption.to_encoder(ChannelWriter::new(tx));
        std::io::copy(&mut Cursor::new(input), &mut encoder).unwrap();
        drop(encoder);
        let encrypted: Vec<u8> = rx.iter().collect();

        let (tx, rx) = mpsc::channel();
        let mut decoder = encryption.to_decoder(Cursor::new(encrypted));
        std::io::copy(&mut decoder, &mut ChannelWriter::new(tx)).unwrap();
        let output: Vec<u8> = rx.iter().collect();

        assert_eq!(output.as_slice(), input);
    }
}
