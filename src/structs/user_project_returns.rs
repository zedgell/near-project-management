use crate::structs::project::Project;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserProjectsReturn {
    pub(crate) in_progress: Vec<Project>,
    pub(crate) complete: Vec<Project>,
}

impl UserProjectsReturn {
    pub fn new() -> UserProjectsReturn {
        UserProjectsReturn {
            in_progress: Vec::new(),
            complete: Vec::new(),
        }
    }
}
