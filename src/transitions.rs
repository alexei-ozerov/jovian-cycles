use crate::models::PracticeSessionData;
use log::error;
use serde::{Deserialize, Serialize};

/*
 * Type State
 */
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum SessionStates {
    Waiting,
    RequestingNewKey,
    Working,
    Resting,
    Finishing,
}

#[serde(default)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PracticeSessionState {
    #[serde(skip)]
    pub finished_flag: bool,
    #[serde(skip)]
    pub note_name_list: Vec<String>,
    #[serde(skip)]
    pub session_state: SessionStates,
    #[serde(skip)]
    pub session_data: PracticeSessionData,
}

/*
* Transition States
*
* New Key -> Working
* New Key -> Resting
* Working -> Resting
*
* All -> Finishing
*/

impl Default for PracticeSessionState {
    fn default() -> Self {
        PracticeSessionState {
            finished_flag: false,
            note_name_list: vec![
                "C".to_owned(),
                "C#".to_owned(),
                "D".to_owned(),
                "Eb".to_owned(),
                "E".to_owned(),
                "F".to_owned(),
                "F#".to_owned(),
                "G".to_owned(),
                "Ab".to_owned(),
                "A".to_owned(),
                "Bb".to_owned(),
                "B".to_owned(),
            ],
            session_state: SessionStates::Waiting,
            session_data: PracticeSessionData::new(),
        }
    }
}

impl PracticeSessionState {
    // (Requesting New Key) Transition function
    pub fn to_requesting_new_key(&mut self) {
        self.session_state = SessionStates::RequestingNewKey;
        self.session_data
            .get_timestamp("Requesting New Key".to_string());
    }

    // (Requesting New Key) State function
    pub fn requesting_new_key(&mut self) {
        self.session_data.get_new_key();
    }

    // (Working) Transition function
    pub fn to_working(&mut self) {
        self.session_state = SessionStates::Working;
        self.session_data.get_timestamp("Working".to_string());
    }

    // (Working) State function
    pub fn working(&mut self) {
        match self.session_data.clone().increment_key_repetition() {
            Ok(data) => {
                self.session_data = data;
            }
            Err(e) => {
                error!("{:#?}", e);
                self.session_state = SessionStates::RequestingNewKey;
            }
        };
    }

    // (Resting) Transition function
    pub fn to_resting(&mut self) {
        self.session_state = SessionStates::Resting;

        self.session_data.get_timestamp("Resting".to_string());
    }

    // (Finishing) Transition function
    pub fn to_finishing(&mut self) {
        self.session_state = SessionStates::Finishing;
        self.session_data.get_timestamp("Finishing".to_string());
    }

    // (Finishing) State function
    pub fn finishing(&mut self) {
        self.session_data.receipt = Some(self.session_data.construct_receipt());
    }
}
