use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

use num_bigint_dig::{BigInt, BigUint};
use serde::{Deserialize, Serialize};
use serde_json;

pub const KEY_STORAGE_PATH: &str = "./key_storage.json";

#[derive(Serialize, Deserialize)]
pub struct Storage {
    key_pairs: HashMap<usize, KeyPair>,
}

impl Storage {
    pub fn new() -> Self {
        let key_pair_map = HashMap::new();
        Self {
            key_pairs: key_pair_map,
        }
    }

    pub fn add_key_pair(&mut self, key_pair: KeyPair) {
        let last_id = self.key_pairs.len();
        self.key_pairs.insert(last_id, key_pair);
    }

    pub fn get_key_pair_by_id(&self, id: usize) -> Option<&KeyPair> {
        self.key_pairs.get(&id)
    }

    pub fn get_key_pairs(&self) -> &HashMap<usize, KeyPair> {
        &self.key_pairs
    }

    pub fn delete_key_pair_by_id(&mut self, id: usize) -> Option<KeyPair> {
        let removed_key_pair = self.key_pairs.remove(&id);

        if removed_key_pair.is_some() {
            let mut new_key_pair_map = HashMap::new();
            let mut new_id = 0;
            for (_, key_pair) in self.key_pairs.iter() {
                new_key_pair_map.insert(new_id, key_pair.clone());
                new_id += 1;
            }
            self.key_pairs = new_key_pair_map;
        }

        removed_key_pair
    }

    pub fn load_storage() -> Self {
        Self::load_storage_from_file(KEY_STORAGE_PATH)
    }

    pub fn load_storage_from_file(path: &str) -> Self {
        if !Path::new(&path).exists() {
            fs::File::create(&path).unwrap();
        }

        if Path::new(&path).metadata().unwrap().len() == 0 {
            Self::new().save_storage_to_path(path);
        }

        let mut storage: Self = Self::new();
        let file = fs::File::open(&path);

        if let Ok(file) = file {
            let reader = std::io::BufReader::new(file);
            let storage_from_file: Storage = serde_json::from_reader(reader).unwrap();
            storage.key_pairs = storage_from_file.key_pairs;
        }
        storage
    }

    pub fn save_storage(&self) {
        self.save_storage_to_path(KEY_STORAGE_PATH)
    }

    pub fn save_storage_to_path(&self, path: &str) {
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(serde_json::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct KeyPair {
    pub name: String,
    pub p: BigUint,
    pub q: BigUint,
    pub modulus: BigUint,
    pub e: BigUint,
    pub d: BigInt,
}

impl KeyPair {
    pub fn new(
        name: String,
        p: BigUint,
        q: BigUint,
        modulus: BigUint,
        e: BigUint,
        d: BigInt,
    ) -> Self {
        KeyPair {
            name,
            p,
            q,
            modulus,
            e,
            d,
        }
    }
}
