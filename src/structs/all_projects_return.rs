use crate::structs::project::Project;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AllProjectsReturn {
    pub(crate) not_started: Vec<Project>,
    pub(crate) in_progress: Vec<Project>,
    pub(crate) complete: Vec<Project>,
}

impl AllProjectsReturn {
    pub fn new() -> AllProjectsReturn {
        AllProjectsReturn {
            not_started: Vec::new(),
            in_progress: Vec::new(),
            complete: Vec::new(),
        }
    }
}
