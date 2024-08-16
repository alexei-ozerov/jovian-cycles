use crate::transitions::{PracticeSessionState, SessionStates};

use log::{debug, error, info};

pub fn match_states(state: &mut PracticeSessionState) {
    match state.session_state {
        SessionStates::RequestingNewKey => {
            state.requesting_new_key();
        }
        SessionStates::Working => {
            state.working();
        }
        SessionStates::Resting => {
            // TODO: (ozerova) - Implement resting features.
            //                   1) Stop working timer (drop RESTING timestamp)
            //                   2) Wait for input to transition state to working, requesting
            //                      new key, or finishing
            //                   3) Transition state
        }
        SessionStates::Finishing => {
            state.finishing();
            match state.session_data.receipt.clone() {
                Some(r) => {
                    // Check if timestamps exist
                    match r.time_stamp_archive {
                        Some(a) => {
                            info!("{:#?}", a);
                        }
                        None => {
                            error!("No timestamps found.");
                        }
                    };

                    // Check if key data exists
                    match r.key_data_archive {
                        Some(a) => {
                            info!("{:#?}", a);
                        }
                        None => {
                            error!("No key data found.");
                        }
                    };
                }
                None => {
                    error!("No receipt found.");
                }
            }

            info!("Gracefully exiting practice session.");
            state.finished_flag = true;
        }
        SessionStates::Waiting => {
            debug!("Waiting for a request for a new key or quit.");
        }
    };
}
