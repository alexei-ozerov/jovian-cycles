use crate::models::{PracticeSessionData, TimeCode};

use log::{debug, error, info};
use serde::{Deserialize, Serialize};

/*
 * Type State
 */
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum SessionStates {
    Waiting,
    RequestingNewKey,
    SkippingKey,
    Working,
    Resting,
    Finishing,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct PracticeSessionState {
    #[serde(skip)]
    pub theme: catppuccin_egui::Theme,
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
            theme: catppuccin_egui::LATTE,
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
            .set_timestamp("Requesting New Key".to_string());
        self.session_data.push_timestamp();
    }

    // (Requesting New Key) State function
    pub fn requesting_new_key(&mut self) {
        self.session_data.get_new_key();
    }

    pub fn to_skipping_key(&mut self) {
        self.session_state = SessionStates::SkippingKey;
    }

    pub fn skipping_key(&mut self) {
        self.decrement_key();

        info!(
            "Pre Truncation: {:#?}",
            self.session_data.practice_session_history
        );

        // Remove last history
        match self.session_data.practice_session_history.clone() {
            None => {}
            Some(mut history) => {
                let length = history.len();
                if length > 1 {
                    history.truncate(length - 1);
                    self.session_data.practice_session_history = Some(history);
                }
            }
        };

        info!(
            "Post Truncation: {:#?}",
            self.session_data.practice_session_history
        );
    }

    pub fn to_waiting(&mut self) {
        self.session_state = SessionStates::Waiting;
    }

    // (Working) Transition function
    pub fn to_working(&mut self) {
        self.session_state = SessionStates::Working;
        self.session_data.set_timestamp("Working".to_string());
        self.session_data.push_timestamp();
    }

    // (Working) State functions
    fn increment_key(&mut self) {
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

    pub fn decrement_key(&mut self) {
        match self.session_data.clone().decrement_key_repetition() {
            Ok(data) => {
                self.session_data = data;
            }
            Err(e) => {
                error!("{:#?}", e);
                self.session_state = SessionStates::RequestingNewKey;
            }
        };
    }

    pub fn working(&mut self) {
        match self.session_data.clone().practice_session_history {
            None => {}
            Some(history) => {
                let length = history.len();
                let previous_state_time_code = &history[length - 2];
                let previous_state_name = &previous_state_time_code.0;

                debug!("{:#?}", length);
                debug!("{:#?}", previous_state_time_code);
                debug!("{:#?}", previous_state_name);

                if previous_state_name == "Requesting New Key" {
                    self.increment_key();
                };
            }
        };

        self.session_data.receipt = Some(self.session_data.construct_receipt());
    }

    // (Resting) Transition function
    pub fn to_resting(&mut self) {
        self.session_state = SessionStates::Resting;
        self.session_data.set_timestamp("Resting".to_string());
        self.session_data.push_timestamp();
    }

    // (Finishing) Transition function
    pub fn to_finishing(&mut self) {
        self.session_state = SessionStates::Finishing;
        self.session_data.set_timestamp("Finishing".to_string());
        self.session_data.push_timestamp();
    }

    // (Finishing) State function
    pub fn finishing(&mut self) {
        self.session_data.receipt = Some(self.session_data.construct_receipt());
    }
}
