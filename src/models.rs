use chrono::Utc;
use color_eyre::eyre::{eyre, Result};
use log::{debug, info};
use serde::{Deserialize, Serialize};

/*
 * Generic Data Types
 */
pub type TimeCode = (String, i64);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct KeyData {
    pub nid: usize,
    repetitions: i32,
    weight: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeysMap(Vec<KeyData>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub key_data_archive: Option<KeysMap>,
    pub time_stamp_archive: Option<Vec<TimeCode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeSessionData {
    practice_session_history: Option<Vec<TimeCode>>, // Aggregate practice session time data
    all_keys_map: KeysMap, // Aggregate data about keys, repetitions, and probability
    pub current_key_data: Option<KeyData>, // Data associated with current key
    start_timestamp: Option<TimeCode>, // State name and start timestamp
    pub receipt: Option<Receipt>, // Receipt of practice session given when process finishes
}

impl PracticeSessionData {
    pub fn new() -> Self {
        let mut keys_map_vec = Vec::new();
        for id in 0..12 {
            let keys_map_data = KeyData {
                nid: id,
                repetitions: 0,
                weight: 100,
            };
            keys_map_vec.push(keys_map_data);
        }

        PracticeSessionData {
            practice_session_history: None,
            all_keys_map: KeysMap(keys_map_vec),
            current_key_data: None,
            start_timestamp: None,
            receipt: None,
        }
    }

    pub fn get_new_key(&mut self) {
        let mut probabilities: Vec<(usize, i32)> = Vec::new();
        self.all_keys_map.0.iter().for_each(|key_data| {
            probabilities.push((key_data.nid, key_data.weight));
        });

        // TODO: (ozerova) - Research and implement weighted random algorithm to select a new key
        probabilities.iter().for_each(|idp_pair| {
            //info!("{:#?}", idp_pair);
        });

        self.current_key_data = Some(self.clone().all_keys_map.0[11]);
    }

    pub fn increment_key_repetition(mut self) -> Result<Self> {
        match self.current_key_data {
            Some(ref mut data) => {
                data.repetitions += 1;
                debug!("{:#?}", self.current_key_data);
                Ok(self)
            }
            None => {
                let msg = eyre!("Unable to increment repetition count for the current key as a current key has not been set. Will attempt to set the state back to RequestingNewKey to try again.");
                Err(msg)
            }
        }
    }

    pub fn get_timestamp(&mut self, state_name: String) {
        let dt = Utc::now();
        let timestamp: i64 = dt.timestamp();

        self.start_timestamp = Some((state_name, timestamp));
    }

    pub fn construct_receipt(&self) -> Receipt {
        Receipt {
            key_data_archive: Some(self.all_keys_map.clone()),
            time_stamp_archive: self.practice_session_history.clone(),
        }
    }
}
