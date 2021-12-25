use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

near_sdk::setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Status {
    Complete,
    PendingFinalApproval,
    InProgress,
    NotStarted,
    PendingWorkerApproval,
    Created,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Project {
    pub(crate) id: String,
    pub(crate) github_issue_link: String,
    pub(crate) description: String,
    pub(crate) reward: u128, // The reward is measured in yoctoNEAR. One NEAR is 10**24 yoctoNEAR
    pub(crate) status: Status,
    pub(crate) worker: Option<AccountId>,
    pub(crate) project_owner: AccountId
}

impl Project {
    pub(crate) fn new(
        id: String,
        github_issue_link: String,
        description: String,
        reward: u128,
        project_owner: AccountId
    ) -> Project {
        Project {
            id,
            github_issue_link,
            description,
            reward,
            status: Status::Created,
            worker: None,
            project_owner
        }
    }
}
